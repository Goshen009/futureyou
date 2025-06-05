use rand::Rng;
use redis::{AsyncCommands, Client};

use crate::responses::ApiError;
use super::Letter;

pub struct Redis(Client);

impl Redis {
    pub fn initialize() -> Self {
        let redis_url = std::env::var("REDIS_URL").expect("No REDIS_URL set");
        let client = Client::open(redis_url).expect("Failed to connect to Redis");

        Self(client)
    }

    pub async fn store_future_letter(&self, letter: &Letter) -> Result<String, ApiError> {
        let mut con = self.0.get_multiplexed_async_connection().await?;

        let key = format!("pending_verification:{}", letter.writer_email);

        let rate = 0;
        let otp = rand::rng().random_range(100_000..1_000_000).to_string();
        let letter_json = serde_json::to_string(letter)?;
        
        let _: () = redis::pipe()
            .hset(&key, "otp", &otp)
            .hset(&key, "rate", rate.to_string())
            .hset(&key, "letter", letter_json)
            .expire(&key, 3600)
            .query_async(&mut con)
            .await?;

        Ok(otp)
    }

    pub async fn validate_otp(&self, email: String, submitted_otp: String) -> Result<Letter, ApiError> {
        let mut con = self.0.get_multiplexed_async_connection().await?;

        let key = format!("pending_verification:{}", email);
  
        let stored_otp: Option<String> = con.hget(&key, "otp").await?;
        let stored_otp = stored_otp.ok_or(ApiError::VerificationSessionExpired)?;

        if stored_otp != submitted_otp {
            let new_rate: i32 = con.hincr(&key, "rate", 1).await?;
            if new_rate >= 5 {
                // but first, try to send em an email sha
                // still atempt to send an email to let em know they failed
                // but their message is still here

                let _: () = con.del(&key).await?;
                return Err(ApiError::TooManyOTPAttempts);
            }
            return Err(ApiError::InvalidOtp(5 - new_rate));
        }

        let letter_json: String = con.hget(&key, "letter").await?;
        let letter = serde_json::from_str::<Letter>(&letter_json)?;

        let _: () = con.del(&key).await?;

        Ok(letter)
    }
}
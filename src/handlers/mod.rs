use std::sync::Arc;

use axum::{body::Body, extract::{Request, State}, routing::{get, post}, Json, Router};
use chrono::Duration;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::{appstate::{AppState, Redis, ResendKit, WebhookSecrets}, responses::{ApiError, ApiResponse}, Database};

pub fn get_routes() -> Router<AppState> {
    Router::new()
        .route("/ping", get(ping))
        .route("/send-future-letter", post(set_future_letter))
        .route("/verify-otp", post(verify_otp))
        .route("/resend/webhook/bounced", post(bounced_webhook))
        .route("/resend/webhook/delivered", post(delivered_webhook))
}

pub async fn ping() {
    println!("Pinged!");
}

#[derive(Serialize, Deserialize, Validate)]
pub struct FutureLetterDetails {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(email(message = "This is not a valid email"))]
    pub email: String,
    #[validate(length(min = 5, max = 1000, message = "Message must between 5 and 1000 letters"))]
    pub message: String,
    #[validate(custom(function = "valid_delivery_date"))]
    pub delivery_date: String
}

fn valid_delivery_date(delivery_date: &str) -> Result<(), validator::ValidationError> {
    use chrono::{Utc, DateTime};

    let delivery_date = delivery_date
        .parse::<DateTime<Utc>>()
        .map_err(|_| validator::ValidationError::new("Invalid datetime")
            .with_message("Delivery date is not a valid datetime".into()))?;

    let time_offset = Utc::now() + Duration::days(1);
    if delivery_date < time_offset {
        return Err(validator::ValidationError::new("Delivery date too soon")
            .with_message("Delivery date must be at least 24 hours into the future".into()));
    }

    Ok(())
}

async fn set_future_letter(
    State(resendkit): State<Arc<ResendKit>>,
    State(redis): State<Arc<Redis>>,
    Json(future_letter_details): Json<FutureLetterDetails>
) -> Result<ApiResponse<()>, ApiError> {

    tracing::info!("Gotten into the handler");

    future_letter_details.validate()?;

    tracing::info!("Validated");

    let letter = crate::appstate::Letter::from(future_letter_details);
    let otp = redis.store_future_letter(&letter).await?;

    tracing::info!("Gotten past redis");

    resendkit.send_verification_email(&letter.writer_email, otp).await?;

    tracing::info!("Finally sent verification email");

    Ok(ApiResponse(StatusCode::OK, Json(())))
}



#[derive(Serialize, Deserialize, Validate)]
pub struct VerifyOTPDetails {
    #[validate(email(message = "This is not a valid email"))]
    pub email: String,
    #[validate(length(equal = 6, message = "OTP must be 6 digits"))]
    pub otp: String,
}

pub async fn verify_otp(
    State(redis): State<Arc<Redis>>,
    State(db): State<Arc<Database>>,
    Json(verify_otp_details): Json<VerifyOTPDetails>
) -> Result<ApiResponse<()>, ApiError> {

    verify_otp_details.validate()?;

    tracing::info!("OTP validated");

    let letter = redis.validate_otp(verify_otp_details.email, verify_otp_details.otp).await?;

    tracing::info!("Validated OTP");

    db.insert_letter(letter).await?;

    tracing::info!("Inserted into DB");

    Ok(ApiResponse(StatusCode::CREATED, Json(())))
}

#[derive(Serialize, Deserialize)]
pub struct ResendWebhookPayload {
    pub data: EmailDeliveryData,
}

#[derive(Serialize, Deserialize)]
pub struct EmailDeliveryData {
    pub email_id: String,
}

pub async fn delivered_webhook (
    State(db): State<Arc<Database>>,
    State(webhook_secrets): State<Arc<WebhookSecrets>>,
    req: Request<Body>,
) -> Result<StatusCode, StatusCode> {

    println!("Webhook delivered for delivered");

    let headers = req.headers().clone();
    let body = axum::body::to_bytes(req.into_body(), usize::MAX)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let wh = svix::webhooks::Webhook::new(&webhook_secrets.delivered_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    wh.verify(&body, &headers)
        .map_err(|_| StatusCode::FORBIDDEN)?;

    let delivered_payload = serde_json::from_slice::<ResendWebhookPayload>(&body)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    println!("Webhook verified for email with id {}", &delivered_payload.data.email_id);

    db.future_letter_delivered(&delivered_payload.data.email_id).await
        .inspect_err(|e| eprintln!("Error deleting delivered email for email id {}: {}", delivered_payload.data.email_id, e))
        .ok();

    Ok(StatusCode::OK)
}

pub async fn bounced_webhook (
    State(webhook_secrets): State<Arc<WebhookSecrets>>,
    req: Request<Body>,
) -> Result<StatusCode, StatusCode>{
    println!("Webhook delivered for bounced");

    let headers = req.headers().clone();
    let body = axum::body::to_bytes(req.into_body(), usize::MAX)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let wh = svix::webhooks::Webhook::new(&webhook_secrets.bounced_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    wh.verify(&body, &headers)
        .map_err(|_| StatusCode::FORBIDDEN)?;

    let delivered_payload = serde_json::from_slice::<ResendWebhookPayload>(&body)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    eprintln!("This letter with email id {} could not be deliverd. It was bounced", delivered_payload.data.email_id);

    Ok(StatusCode::OK)
}
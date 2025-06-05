use std::sync::Arc;
use axum_macros::FromRef;

mod redis;
mod database;
mod resendkit;

pub use redis::Redis;
pub use database::{Database, Letter};
pub use resendkit::{ResendKit, ScheduledLetter};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub redis: Arc<Redis>,
    pub database: Arc<Database>,
    pub resendkit: Arc<ResendKit>,
    pub webhook_secrets: Arc<WebhookSecrets>,
}

impl AppState {
    pub fn new(database: Database) -> Self {
        Self {
            database: Arc::new(database),
            redis: Arc::new(Redis::initialize()),
            resendkit: Arc::new(ResendKit::initialize()),
            webhook_secrets: Arc::new(WebhookSecrets::initialize())
        }
    }
}

#[derive(Clone)]
pub struct WebhookSecrets {
    pub delivered_secret: String,
    pub bounced_secret: String,
}

impl WebhookSecrets {
    pub fn initialize() -> Self {
        Self {
            delivered_secret: std::env::var("RESEND_WEBHOOK_DELIVERED_SECRET").expect("No RESEND_WEBHOOK_DELIVERED_SECRET set"),
            bounced_secret: std::env::var("RESEND_WEBHOOK_BOUNCED_SECRET").expect("No RESEND_WEBHOOK_BOUNCED_SECRET set")
        }
    }
}
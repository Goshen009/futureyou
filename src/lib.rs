use axum::{http::{header, HeaderValue, Method}, Router};
use tower_http::{cors::CorsLayer, trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer}};

mod handlers;
mod appstate;

pub use appstate::{AppState, Database, ResendKit};
use tracing::Level;

pub fn load_app(database: Database) -> Router {
    let origin_url = std::env::var("FRONTEND_URL").expect("FRONTEND_URL must be set");

    Router::new()
        .merge(handlers::get_routes())

        .layer(CorsLayer::new()
            .allow_origin(origin_url.parse::<HeaderValue>().unwrap())
            .allow_headers([header::CONTENT_TYPE])
            .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE]) 
        )

        .layer(TraceLayer::new_for_http()
            .on_request(DefaultOnRequest::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO))
        )

        .with_state(AppState::new(database))
}

mod responses {
    use std::collections::HashMap;

    use axum::{http::StatusCode, response::IntoResponse, Json};
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use thiserror::Error;

    pub struct ApiResponse<T: Serialize>(pub StatusCode, pub Json<T>);

    impl <T: Serialize> IntoResponse for ApiResponse<T> {
        fn into_response(self) -> axum::response::Response {
            (self.0, self.1).into_response()
        }
    }

    #[derive(Debug, Error)]
    pub enum ApiError {
        #[error("Validation failed: {0}")]
        Validation(#[from] validator::ValidationErrors),

        #[error("Redis error: {0}")]
        Redis(#[from] redis::RedisError),

        #[error("JWT error: {0}")]
        Jwt(#[from] jsonwebtoken::errors::Error),

        #[error("Database error: {0}")]
        Mongo(#[from] mongodb::error::Error),

        #[error("Reqwest error: {0}")]
        Request(#[from] reqwest::Error),

        #[error("Resend error: {0}")]
        Resend(#[from] resend_rs::Error),

        #[error("Serialization error: {0}")]
        Serialization(#[from] serde_json::Error),

        #[error("Verification session expired or wrong email")]
        VerificationSessionExpired,

        #[error("Too many attempts")]
        TooManyOTPAttempts,

        #[error("Invalid OTP. {0} tries remaining")]
        InvalidOtp(i32),

        #[error("Unexpected error: {0}")]
        Other(String),
    }

    impl IntoResponse for ApiError {
        fn into_response(self) -> axum::response::Response {
            let status = self.status_code();
            let body = self.body(status);
            (status, body).into_response()
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct ErrorResponse {
        pub status: u16,
        pub detail: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub errors: Option<HashMap<String, String>>
    }

    impl ApiError {
        const fn status_code(&self) -> StatusCode {
            match self {
                ApiError::Validation(_) => StatusCode::BAD_REQUEST,
                ApiError::TooManyOTPAttempts => StatusCode::TOO_MANY_REQUESTS,
                ApiError::VerificationSessionExpired | ApiError::InvalidOtp(_) => StatusCode::BAD_REQUEST,
                _ => StatusCode::INTERNAL_SERVER_ERROR
            }
        }

        fn body(&self, status: StatusCode) -> Json<Value> {
            let mut errors = None;

            if let ApiError::Validation(e) = self {
                errors = Some(validation_errors_to_map(e));
            }

            if let ApiError::InvalidOtp(remaining) = self {
                let mut map = HashMap::new();
                let otp_error_message = format!("Invalid OTP {} tries remaining", remaining);

                map.insert("otp".to_string(), otp_error_message);

                errors = Some(map);
            }

            let response = ErrorResponse {
                status: status.as_u16(),
                detail: self.to_string(),
                errors,
            };

            Json(serde_json::to_value(response).unwrap())
        }
    }

    fn validation_errors_to_map(errors: &validator::ValidationErrors) -> HashMap<String, String> {
        let mut map = HashMap::new();
    
        for (field, field_errors) in errors.field_errors() {
            if let Some(first_error) = field_errors.first() {
                let message = first_error
                    .message
                    .clone()
                    .unwrap_or_else(|| format!("Invalid value for {}", field).into());
                map.insert(field.to_string(), message.to_string());
            }
        }
    
        map
    }
}
use mongodb::bson::DateTime;
use serde::{Serialize, Deserialize};
use crate::handlers::FutureLetterDetails;

#[derive(Serialize, Deserialize)]
pub struct Letter {
    #[serde(rename = "_id")]
    pub id: String,
    pub state: State,
    pub message: String,
    pub due_date: DateTime,
    pub created_at: DateTime,
    pub writer_name: String,
    pub writer_email: String,
    pub email_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum State {
    Pending,
    Scheduled,
    Sent,
    Failed,
}

impl From<FutureLetterDetails> for Letter {
    fn from(value: FutureLetterDetails) -> Self {
        Self {
            id: uuid::Uuid::now_v7().to_string(),
            state: State::Pending,
            message: value.message,
            due_date: DateTime::parse_rfc3339_str(value.delivery_date).unwrap(),
            created_at: DateTime::now(),
            writer_name: value.name,
            writer_email: value.email,
            email_id: None
        }
    }
}
pub mod models;
pub use models::Letter;

use mongodb::{bson::doc, error::Error, options::{UpdateOneModel, WriteModel}, results::{DeleteResult, InsertOneResult, SummaryBulkWriteResult}, Client, Collection, Cursor, Database as MongoDatabase};
use super::ScheduledLetter;

#[derive(Clone)]
pub struct Database {
    db: MongoDatabase
}

impl Database {
    pub async fn connect() -> Result<Self, Error> {
        let uri = std::env::var("DATABASE_URI").expect("DATABASE_URI must be set");
        let client = Client::with_uri_str(uri).await?;
        let db = client.database("future-me");

        Ok(Self{ db })
    }

    pub fn letters(&self) -> Collection<Letter> {
        self.db.collection::<Letter>("letters")
    }

    pub async fn insert_letter(&self, letter: Letter) -> Result<InsertOneResult, Error> {
        self.letters().insert_one(letter).await
    }

    pub async fn fetch_letters_to_be_scheduled(&self) -> Result<Cursor<Letter>, Error> {
        let query = doc! {
            "state": "Pending",
            "due_date": { "$lt": bson::DateTime::from_chrono(chrono::Utc::now() + chrono::Duration::days(29))}
        };

        self.letters().find(query).await
    }
    
    pub async fn bulk_update_scheduled_letters(&self, scheduled_letters: &[ScheduledLetter]) -> Result<SummaryBulkWriteResult, Error> {
        let updates = scheduled_letters.iter()
            .map(|s| {
                WriteModel::UpdateOne(
                    UpdateOneModel::builder()
                        .namespace(self.letters().namespace())
                        .filter(doc! { "_id": &s.letter_id } )
                        .update(doc! { "$set" : {
                            "state": "Scheduled",
                            "email_id": &s.email_id,
                        }})
                        .build()
                )
            });
        
        self.db.client().bulk_write(updates).await
    }

    pub async fn future_letter_delivered(&self, email_id: &String) -> Result<DeleteResult, Error> {
        self.letters().delete_one(doc! { "email_id" : email_id } ).await
    }
}
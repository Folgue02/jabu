use chrono::Utc;
use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;

/// Information about an artifact.
#[derive(Debug, PartialEq, FromRow, Serialize, Deserialize)]
pub struct ArtifactInfo {
    pub description: String,
    pub creation_date: chrono::NaiveDateTime,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct AuthorInfo {
    pub author: String,
    pub creation_date: chrono::DateTime<Utc>,
    pub artifact_count: i64,
}

impl AuthorInfo {
    pub fn new(author: String, creation_date: chrono::DateTime<Utc>, artifact_count: i64) -> Self {
        Self {
            author,
            creation_date,
            artifact_count
        }
    }
}

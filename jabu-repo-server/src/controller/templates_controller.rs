use askama::Template;
use chrono::Utc;
use serde::Deserialize;

use super::artifacts::{ArtifactsRow, GeneralizedArtifactInfo};

#[derive(Template)]
#[template(path = "index.html")]
pub struct HomePage {
    pub artifact_count: i32,
}

impl HomePage {
    pub fn new(artifact_count: i32) -> Self {
        Self { artifact_count }
    }
}

#[derive(Template, Deserialize)]
#[template(path = "search.html")]
pub struct SearchPage {
    pub search_term: String,
    pub results: Vec<GeneralizedArtifactInfo>,
}

impl SearchPage {
    pub fn new(search_term: String, results: Vec<GeneralizedArtifactInfo>) -> Self {
        Self {
            search_term,
            results,
        }
    }
}

#[derive(Template, Deserialize)]
#[template(path = "artifact.html")]
pub struct ArtifactPage {
    pub versions: Vec<ArtifactsRow>,
    pub tags: Vec<String>,
    pub first_release_date: chrono::DateTime<Utc>,
    pub latest_release_date: chrono::DateTime<Utc>,
}

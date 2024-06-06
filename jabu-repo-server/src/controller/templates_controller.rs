use askama::Template;
use serde::Deserialize;

use super::artifacts::ArtifactsRow;

#[derive(Template)]
#[template(path = "index.html")]
pub struct HomePage { 
    pub artifact_count: i32
}

impl HomePage {
    pub fn new(artifact_count: i32) -> Self {
        Self {
            artifact_count
        }
    }
}

#[derive(Template, Deserialize)]
#[template(path = "search.html")]
pub struct SearchPage { 
    pub search_term: String,
    pub results: Vec<ArtifactsRow>,

}

impl SearchPage {
    pub fn new(search_term: String, results: Vec<ArtifactsRow>) -> Self {
        Self {
            search_term, results
        }
    }
}

/*
 * TODO: Find out how blocks and inheritance works.
#[derive(Template)]
#[template(path = "footer.html", block = "footer")]
pub struct FooterBlock;
*/

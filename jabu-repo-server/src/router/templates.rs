use askama_axum::IntoResponse;
use axum::{extract::{Query, State}, http::StatusCode};
use serde::Deserialize;

use crate::controller::{
    self,
    templates_controller,
};

use super::AppState;

pub async fn index(State(app_state): State<AppState>) -> impl IntoResponse {
    let artifact_count = controller::artifacts::count_artifacts(&app_state.database).await;

    templates_controller::HomePage::new(artifact_count).into_response()
}

#[derive(Deserialize)]
pub struct SearchQuery {
    search_term: String
}

pub async fn search(
    Query(search_query): Query<SearchQuery>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    // Template
    let results = match controller::artifacts::search_artifacts(&search_query.search_term, &app_state.database).await {
        Ok(query_results) => query_results,
        Err(e) => {
            let err_msg = format!("Internal server error when performing search query: {e}");
            log::error!("{err_msg}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    };
    let search_page = templates_controller::SearchPage::new(search_query.search_term, results);
    search_page.into_response()
}

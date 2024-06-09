use askama_axum::IntoResponse;
use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use jabu_config::model::ArtifactSpec;
use serde::Deserialize;

use crate::{controller::{self, artifacts::{all_artifact_versions, first_latest_release_date}, tags::tags_of_artifact, templates_controller::{self, ArtifactPage}}, model::error::ApiError};

use super::AppState;

pub async fn index(State(app_state): State<AppState>) -> impl IntoResponse {
    let artifact_count = controller::artifacts::count_artifacts(&app_state.database).await;

    templates_controller::HomePage::new(artifact_count).into_response()
}

#[derive(Deserialize)]
pub struct SearchQuery {
    search_term: String,
}

pub async fn search(
    Query(search_query): Query<SearchQuery>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    // Template
    let results = match controller::artifacts::search_artifacts(
        &search_query.search_term,
        &app_state.database,
    )
    .await
    {
        Ok(query_results) => query_results,
        Err(e) => {
            let err_msg = format!("Internal server error when performing search query: {e}");
            log::error!("{err_msg}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };
    let search_page = templates_controller::SearchPage::new(search_query.search_term, results);
    search_page.into_response()
}

#[derive(Deserialize)]
pub struct ArtifactId {
    author: String,
    id: String,
}

impl Into<ArtifactSpec> for ArtifactId {
    fn into(self) -> ArtifactSpec {
        ArtifactSpec {
            author: self.author,
            artifact_id: self.id,
            version: "0.0.0".to_string()
        }
    }
}

pub async fn artifact(
    Query(artifact_id): Query<ArtifactId>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let artifact_spec: ArtifactSpec = artifact_id.into();
    let all_versions = match all_artifact_versions(&artifact_spec, &app_state.database).await {
        Ok(versions) => versions,
        Err(e) => {
            let err_msg = format!("Couldn't get all versions of the artifact {} due to the following error: {e}", &artifact_spec.author);
            log::error!("{err_msg}");
            return ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, err_msg).into_response();
        }
    };

    let tags = match tags_of_artifact(&artifact_spec, &app_state.database).await {
        Ok(tags) => tags,
        Err(e) => {
            let err_msg = format!("Couldn't get all tags of artifact {} due to the following error: {e}", artifact_spec);
            log::error!("{err_msg}");
            return ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, err_msg).into_response();
        }
    };

    let (first_release_date, latest_release_date) = match first_latest_release_date(&artifact_spec, &app_state.database).await {
        Ok((first_date, latest_date)) => (first_date, latest_date),
        Err(e) => {
            let err_msg = format!("Couldn't get the first and last date of upload of artifact {artifact_spec}: {e}");
            log::error!("{err_msg}");
            return ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, err_msg).into_response();
        }
    };

    ArtifactPage {
        versions: all_versions,
        tags,
        first_release_date,
        latest_release_date
    }.into_response()
}

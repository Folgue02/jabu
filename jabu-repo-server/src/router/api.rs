use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    Json,
};
use jabu_config::model::ArtifactSpec;

use crate::config::Config;

pub async fn list_author_artifacts(
    Path(author): Path<String>,
    State(server_config): State<Config>,
) -> Response {
    if let Some(author_artifacts) = server_config.jabu_repo.get_author_artifacts(author) {
        (StatusCode::OK, Json(author_artifacts)).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn list_artifact_versions(
    Path((author, artifact)): Path<(String, String)>,
    State(server_config): State<Config>,
) -> Response {
    match server_config
        .jabu_repo
        .get_artifact_versions(author, artifact)
    {
        Some(versions) => (StatusCode::OK, Json(versions)).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn get_artifact(
    Path((author, artifact_name, version, file_type)): Path<(String, String, String, String)>,
    State(server_config): State<Config>,
) -> Response {
    let artifact_spec = ArtifactSpec::new(author, artifact_name, version);

    if server_config.jabu_repo.exists(&artifact_spec) {
        let item_path = match file_type.as_str() {
            "jar" => server_config.jabu_repo.jar_path(&artifact_spec),
            "jaburon" => server_config.jabu_repo.jaburon_path(&artifact_spec),
            _ => return StatusCode::BAD_REQUEST.into_response(),
        };

        match std::fs::read(&item_path) {
            Ok(item_contents) => (
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, "application/jar".to_string()),
                    (
                        header::CONTENT_DISPOSITION,
                        format!("inline; filename=\"{}\"", artifact_spec.to_string()),
                    ),
                ],
                item_contents,
            )
                .into_response(),
            Err(e) => {
                log::error!(
                    "Error while reading target contents for artifact: {artifact_spec} {e}"
                );
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    } else {
        log::info!("Request to fetch non-existent item: {artifact_spec} of type {file_type}");
        StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn ping() -> impl IntoResponse {
    (StatusCode::OK, Html("<h1>Hello World!</h1>\n"))
}

pub async fn api_fallback() -> (StatusCode, Json<HashMap<&'static str, &'static str>>) {
    let mut hm = HashMap::new();
    hm.insert("status", "error");
    log::info!("Tried to access an unknown route!");
    (StatusCode::NOT_FOUND, Json(hm))
}

use std::collections::HashMap;

use crate::{model::error::ApiError, controller::artifacts};

use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::{header, Response, StatusCode},
    response::{Html, IntoResponse},
    Json,
};
use jabu_config::model::{ArtifactSpec, JabuProject};
use sqlx::Row;

use crate::{
    controller::authors::check_uuid_author,
    model::*,
};

use super::AppState;

pub async fn list_author_artifacts(
    Path(author): Path<String>,
    State(app_state): State<AppState>,
) -> Response<Body> {
    if let Some(author_artifacts) = app_state.config.jabu_repo.get_author_artifacts(author) {
        (StatusCode::OK, Json(author_artifacts)).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn list_artifact_versions(
    Path((author, artifact)): Path<(String, String)>,
    State(app_state): State<AppState>,
) -> Response<Body> {
    match app_state
        .config
        .jabu_repo
        .get_artifact_versions(author, artifact)
    {
        Some(versions) => (StatusCode::OK, Json(versions)).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn get_artifact(
    Path((author, artifact_name, version, file_type)): Path<(String, String, String, String)>,
    State(app_state): State<AppState>,
) -> Response<Body> {
    let artifact_spec = ArtifactSpec::new(author, artifact_name, version);

    if app_state.config.jabu_repo.exists(&artifact_spec) {
        let item_path = match file_type.as_str() {
            "jar" => app_state.config.jabu_repo.jar_path(&artifact_spec),
            "jaburon" => app_state.config.jabu_repo.jaburon_path(&artifact_spec),
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
        if artifacts::does_artifact_exist(&artifact_spec, &app_state.database).await.unwrap_or(false) {
            log::warn!("INCONSISTENCY DETECTED BETWEEN THE DATABASE AND THE REPOSITORY! The artifact appears to be registered in the database, but the repository doesn't contain it.");
        }
        let err_msg = format!("Request to fetch non-existent item: {artifact_spec} of type {file_type}");
        log::info!("{err_msg}");
        ApiError::new(StatusCode::NOT_FOUND, err_msg).into_response()
    }
}

/// Returns information about an author requested, the name and its creation date.
pub async fn get_author_info(
    Path(author_name): Path<String>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    match sqlx::query("SELECT COUNT(*) AS count FROM authors WHERE author = $1")
        .bind(&author_name)
        .fetch_one(&app_state.database)
        .await
    {
        Ok(count) => {
            if count.get::<i64, &str>("count") == 0 {
                log::warn!("Cannot find author with name '{author_name}'.");
                return (
                    StatusCode::NOT_FOUND,
                    generate_api_error(format!(
                        "Cannot retrieve info of author with name '{author_name}'."
                    )),
                )
                    .into_response();
            }
        }
        Err(e) => {
            log::error!("Internal database error when looking for author '{author_name}': {e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                generate_api_error(format!(
                    "Internal database error when retrieving info of author '{author_name}': {e}"
                )),
            )
                .into_response();
        }
    }

    let query_result = sqlx::query("SELECT author, creation_date FROM authors WHERE author = $1")
        .bind(&author_name)
        .fetch_one(&app_state.database)
        .await;

    match query_result {
        Ok(row) => {
            let artifact_count = match sqlx::query(
                "SELECT COUNT(*) AS artifact_count FROM artifacts WHERE author = $1",
            )
            .bind(&author_name)
            .fetch_one(&app_state.database)
            .await
            {
                Ok(result) => result.get::<i64, &str>("artifact_count"),
                Err(e) => {
                    log::error!("Internal database error when counting artifacts from author '{author_name}': {e}");
                    0
                }
            };

            let artifact_info =
                AuthorInfo::new(row.get("author"), row.get("creation_date"), artifact_count);
            (StatusCode::OK, Json(artifact_info)).into_response()
        }
        Err(e) => {
            log::error!(
                "Internal database error when retrieving data of author '{author_name}': {e}"
            );
            (StatusCode::INTERNAL_SERVER_ERROR, generate_api_error(format!("Couldn't retrieve info of author '{author_name}' due to the following error: {e}"))).into_response()
        }
    }
}

pub async fn register_author(
    Path(author_name): Path<String>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    log::info!("Attempting to register author '{author_name}'...");

    match sqlx::query("SELECT COUNT(*) AS count FROM authors WHERE author = $1")
        .bind(&author_name)
        .fetch_one(&app_state.database)
        .await
    {
        Ok(count) => {
            if count.get::<i64, &str>("count") > 0 {
                log::warn!(
                    "Attempt to register author for a second time with name '{author_name}'."
                );
                return (
                    StatusCode::UNAUTHORIZED,
                    generate_api_error(format!(
                        "Attempt to register author for a second time with name '{author_name}'."
                    )),
                )
                    .into_response();
            }
        }
        Err(e) => {
            log::error!("Internal database error when checking the authors table: {e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                generate_api_error(format!(
                    "Internal database error when checking the authors table: {e}"
                )),
            )
                .into_response();
        }
    }

    let new_uuid = uuid::Uuid::new_v4().to_string();

    let query_result = sqlx::query(
        "INSERT INTO authors (author, uuid_key)
                VALUES
                    ($1, $2)",
    )
    .bind(&author_name)
    .bind(&new_uuid)
    .execute(&app_state.database)
    .await;

    match query_result {
        Ok(result) => {
            log::info!(
                "Author '{author_name}' registered in query with '{}' rows affected.",
                result.rows_affected()
            );
            (StatusCode::OK, new_uuid).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            generate_api_error(format!(
                "Internal database error when attempting to register author '{author_name}': {e}"
            )),
        )
            .into_response(),
    }
}

pub async fn get_artifact_info(
    Path((author, artifact_name, version)): Path<(String, String, String)>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, ArtifactInfo>(
        "SELECT description, creation_date 
         FROM artifacts
         WHERE author = $1 AND artifact_id = $2 AND version = $3",
    )
    .bind(author)
    .bind(artifact_name)
    .bind(version)
    .fetch_one(&app_state.database)
    .await
    {
        Ok(row) => (StatusCode::OK, Json(row)).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            generate_api_error(format!("No such artifact: {e}")),
        )
            .into_response(),
    }
}

pub async fn upload_artifact(
    Path((author, artifact_id, version, uuid_author_key)): Path<(String, String, String, String)>,
    State(app_state): State<AppState>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let spec = ArtifactSpec::new(author, artifact_id, version);

    match check_uuid_author(&spec.author, uuid_author_key, &app_state.database).await {
        Ok(exists) => {
            if !exists {
                log::warn!("Failed attempt of uploading artifact '{spec}' with wrong credentials.");
                return (
                    StatusCode::FORBIDDEN,
                    generate_api_error("Wrong credentials."),
                )
                    .into_response();
            }
        }
        Err(e) => {
            let error_msg = format!(
                "Couldn't check validity of author '{}' credentials due to the following error: {}",
                spec.author, e
            );
            log::error!("{}", error_msg);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                generate_api_error(error_msg),
            )
                .into_response();
        }
    }

    let jaburon_contents = match multipart.next_field().await {
        Ok(Some(jaburon_contents)) => match jaburon_contents.bytes().await {
            Ok(jaburon_bytes) => jaburon_bytes.into_iter().collect::<Vec<u8>>(),
            Err(e) => {
                let err_msg =
                    format!("Couldn't read from the multipart bytes of artifact '{spec}': {e}");
                log::error!("{err_msg}");
                return ApiError::new(StatusCode::BAD_REQUEST, err_msg).into_response();
            }
        },
        Err(e) => {
            let error_msg = format!("Couldn't read from jaburon multipart for artifact '{spec}' due to the following error: {e}");
            log::error!("{}", error_msg);
            return (StatusCode::BAD_REQUEST, generate_api_error(error_msg)).into_response();
        }
        _ => {
            // next_field() => None
            let err_msg = format!(
                "Missing multipart for the jaburon contents of the artifact for artifact {spec}"
            );
            log::error!("{err_msg}");
            return ApiError::new(StatusCode::BAD_REQUEST, err_msg).into_response();
        }
    };

    let jar_contents = match multipart.next_field().await {
        Ok(Some(jaburon_contents)) => match jaburon_contents.bytes().await {
            Ok(jaburon_bytes) => jaburon_bytes.into_iter().collect::<Vec<u8>>(),
            Err(e) => {
                let err_msg =
                    format!("Couldn't read from the multipart bytes of artifact '{spec}': {e}");
                log::error!("{err_msg}");
                return ApiError::new(StatusCode::BAD_REQUEST, err_msg).into_response();
            }
        },
        Err(e) => {
            let error_msg = format!("Couldn't read from jaburon multipart for artifact '{spec}' due to the following error: {e}");
            log::error!("{}", error_msg);
            return (StatusCode::BAD_REQUEST, generate_api_error(error_msg)).into_response();
        }
        _ => {
            // next_field() = None
            let err_msg = format!(
                "Missing multipart for the jaburon contents of the artifact for artifact {spec}"
            );
            log::error!("{err_msg}");
            return ApiError::new(StatusCode::BAD_REQUEST, err_msg).into_response();
        }
    };

    let description = match JabuProject::try_from(
        String::from_utf8(jaburon_contents.clone())
            .unwrap_or_default()
            .as_str(),
    ) {
        Ok(jabu_project) => jabu_project.header.description,
        Err(e) => {
            let err_msg = format!("Couldn't parse the jaburon of the artifact '{spec}' that was attempted to be registered due to the following error: {e}");
            log::error!("{err_msg}");
            return ApiError::new(StatusCode::BAD_REQUEST, err_msg).into_response();
        }
    };

    match app_state.config.jabu_repo.save_artifact(
        &spec,
        jar_contents.as_slice(),
        jaburon_contents.as_slice(),
    ) {
        Ok(_) => {
            if let Err(e) = sqlx::query(
                "INSERT INTO artifacts (author, artifact_id, version, description)
                         VALUES ($1, $2, $3, $4)",
            )
            .bind(&spec.author)
            .bind(&spec.artifact_id)
            .bind(&spec.version)
            .bind(description)
            .execute(&app_state.database)
            .await
            {
                let err_msg = format!("Couldn't register artifact ('{spec}') in the database: {e}");
                log::error!("{err_msg}");
                return ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, err_msg).into_response();
            }

            log::info!("New artifact created: '{spec}'");
            StatusCode::OK.into_response()
        }
        Err(e) => {
            let err_msg = format!("Couldn't store artifact '{spec}' in the repository due to the following error: {e}");
            log::error!("{err_msg}");
            return ApiError::new(
                StatusCode::BAD_REQUEST,
                "Something went wrong when storing the artifact in the repository.",
            )
            .into_response();
        }
    }
}

/// Generates a JSON containing an status and a description (`description`).
fn generate_api_error(description: impl Into<String>) -> ApiError {
    ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, description.into())
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

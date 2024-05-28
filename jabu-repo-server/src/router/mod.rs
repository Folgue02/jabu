use axum::{middleware::from_fn, routing::{get, post}, Router};
use sqlx::{Pool, Postgres, PgPool};

use crate::config::Config;

pub mod api;
pub mod middleware;

#[derive(Clone, Debug)]
pub struct AppState {
    pub config: Config,
    pub database: Pool<Postgres>
}

/// Generates the router with all the routes incorporated in it.
pub fn get_app_router(api_router: Router, _: Pool<Postgres>) -> Router {
    Router::new()
        .nest("/api", api_router)
        .fallback(api::api_fallback)
        .layer(from_fn(middleware::logging_middleware))
}


/// Router related to the API
pub fn get_api_router(server_config: Config, database: PgPool) -> Router {
    Router::new()
        .route(
            "/get/:author/:artifact_name/:artifact_version/:file_type",
            get(api::get_artifact),
        )
        .route("/list/:author", get(api::list_author_artifacts))
        .route("/info/:author/:artifact_name/:version", get(api::get_artifact_info))
        .route("/author-info/:author_name", get(api::get_author_info))
        .route("/ping", get(api::ping))
        .route("/list-versions/:author/:artifact", get(api::list_artifact_versions))
        .route("/register-author/:author_name", post(api::register_author))
        .route("/upload/:author/:artifact_id/:version/:uuid_key", post(api::upload_artifact))
        .with_state(AppState { config: server_config, database } )
}

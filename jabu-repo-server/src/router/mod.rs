use axum::{middleware::from_fn, routing::{get, post}, Router};
use sqlx::{Pool, Postgres, PgPool};
use tower_http::services::ServeDir;

use crate::config::Config;

pub mod api;
pub mod middleware;
pub mod templates;

#[derive(Clone, Debug)]
pub struct AppState {
    pub config: Config,
    pub database: Pool<Postgres>
}

/// Generates the router with all the routes incorporated in it.
pub fn get_app_router(app_state: AppState) -> Router {
    let api_router = get_api_router(app_state.config.clone(), app_state.database.clone());
    Router::new()
        .route("/index.html", get(templates::index))
        .route("/search.html", get(templates::search))
        .route("/artifact.html", get(templates::artifact))
        .with_state(app_state)
        .nest("/api", api_router)
        .fallback(api::api_fallback)
        .nest_service("/static/", ServeDir::new("./static"))
        .layer(from_fn(middleware::logging_middleware))
}


/// Router containing the API routes.
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

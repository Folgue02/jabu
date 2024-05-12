use axum::{middleware::from_fn, routing::get, Router};

use crate::config::Config;

pub mod api;
pub mod middleware;

/// Generates the router with all the routes incorporated in it.
pub fn get_app_router(api_router: Router) -> Router {
    Router::new()
        .nest("/api", api_router)
        .fallback(api::api_fallback)
        .layer(from_fn(middleware::logging_middleware))
}

/// Router related to the API
pub fn get_api_router(server_config: Config) -> Router {
    Router::new()
        .route(
            "/get/:author/:artifact_name/:artifact_version/:file_type",
            get(api::get_artifact),
        )
        .route("/list/:author", get(api::list_author_artifacts))
        .route("/ping", get(api::ping))
        .route("/list-versions/:author/:artifact", get(api::list_artifact_versions))
        .with_state(server_config)
}

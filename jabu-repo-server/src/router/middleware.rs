use axum::{body::Body, extract::Request, middleware::Next, response::IntoResponse};

pub async fn logging_middleware(req: Request<Body>, next: Next) -> impl IntoResponse {
    log::info!("REQUEST: {} {}", req.method(), req.uri());
    next.run(req).await
}

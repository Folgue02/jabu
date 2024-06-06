use clap::Parser;
use tokio::net::TcpListener;

mod config;
mod router;
mod controller;
mod model;

const DATABASE_URL: &'static str = "postgres://repository@localhost";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    simple_logger::init().expect("Couldn't initialize the logger.");
    log::info!("Initializing database...");
    let database = controller::init_db().await.expect("Couldn't initialize database");
    log::info!("Done.");

    // ***************************
    // Parse CLI args
    // ***************************
    let cli_config = config::CliArgs::parse();

    // ***************************
    // Create server configuration
    // ***************************
    log::info!(
        "Using '{}' as repository.",
        cli_config.repo_path
    );

    let server_addr = format!("localhost:{}", cli_config.port);
    let tcp_listener = TcpListener::bind(&server_addr).await.unwrap();
    
    // ***************
    // Starting server
    // ***************
    log::info!("Serving on address '{server_addr}'.");
    let server_config: config::Config = config::CliArgs::parse().into();
    axum::serve(
        tcp_listener,
        router::get_app_router(router::AppState { config: server_config, database }),
    )
    .await
    .unwrap();
}

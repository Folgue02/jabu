use sqlx::{Pool, Postgres};

pub mod artifacts;
pub mod error;
pub mod authors;
pub mod templates_controller;
pub mod tags;

pub async fn init_db() -> Result<Pool<Postgres>, sqlx::error::Error> {
    let connection = sqlx::postgres::PgPool::connect("postgres://sqlx_tester:1234@localhost/sqlx_testing").await?;
    sqlx::migrate!("./migrations");
    log::info!("Migration completed!");

    Ok(connection)
}

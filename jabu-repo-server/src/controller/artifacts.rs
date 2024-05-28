use jabu_config::model::ArtifactSpec;
use sqlx::{Pool, Postgres, Row};

/// Registers an artifact into the database, **this doesn't include the process of storing the
/// artifact to the repository**.
pub async fn register_artifact(
    spec: ArtifactSpec,
    description: impl Into<String>,
    database: &Pool<Postgres>,
) -> sqlx::Result<()> {
    log::info!("Registering artifact '{spec}' into the database...");

    sqlx::query(
        "INSERT INTO artifacts 
        (author, artifact_id, version, description) 
        VALUES 
        ($1, $2, $3, $4);",
    )
    .bind(spec.author)
    .bind(spec.artifact_id)
    .bind(spec.version)
    .bind(description.into())
    .execute(database)
    .await?;

    Ok(())
}

/// Checks if the given artifact exists in the `artifacts` table.
pub async fn does_artifact_exist(
    spec: &ArtifactSpec,
    database: &Pool<Postgres>,
) -> sqlx::Result<bool> {
    let query_results = sqlx::query(
        "SELECT COUNT(*) AS count
                FROM artifacts
                WHERE author = $1 AND artifact_id = $2 version = $3;",
    )
    .bind(&spec.author)
    .bind(&spec.artifact_id)
    .bind(&spec.version)
    .fetch_one(database)
    .await?;

    Ok(query_results.get::<i64, &str>("count") > 0)
}

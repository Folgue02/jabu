use askama_axum::IntoResponse;
use chrono::{DateTime, Utc};
use jabu_config::model::ArtifactSpec;
use serde::Deserialize;
use sqlx::{postgres::PgRow, prelude::FromRow, Pool, Postgres, Row};

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
                WHERE author = $1 AND artifact_id = $2 AND version = $3;",
    )
    .bind(&spec.author)
    .bind(&spec.artifact_id)
    .bind(&spec.version)
    .fetch_one(database)
    .await?;

    Ok(query_results.get::<i64, &str>("count") > 0)
}

pub async fn count_artifacts(database: &Pool<Postgres>) -> i32 {
    let query_result = sqlx::query(
        "SELECT COUNT(*) AS count
         FROM artifacts",
    )
    .fetch_one(database)
    .await;

    match query_result {
        Ok(row) => row.get::<i64, &str>("count") as i32,
        Err(e) => {
            log::warn!("Tried to count the number of registered artifacts, but couldn't due to the following error: {e}");
            0
        }
    }
}

#[derive(Deserialize)]
pub struct ArtifactsRow {
    pub spec: ArtifactSpec,
    pub description: String,
    pub creation_date: chrono::DateTime<Utc>,
}

impl FromRow<'_, PgRow> for ArtifactsRow {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        let spec = ArtifactSpec::new(
            row.try_get::<String, &str>("author")?,
            row.try_get::<String, &str>("artifact_id")?,
            row.try_get::<String, &str>("version")?,
        );
        let description = row.try_get::<String, &str>("description")?;
        let creation_date = row.try_get::<DateTime<Utc>, &str>("creation_date")?;

        Ok(ArtifactsRow {
            spec,
            description,
            creation_date,
        })
    }
}

#[derive(Deserialize)]
pub struct GeneralizedArtifactInfo {
    pub search_term: String,
    pub author: String,
    pub artifact_id: String,
    pub latest_version: String,
    pub description: String,
    pub first_date: chrono::DateTime<Utc>,
    pub latest_date: chrono::DateTime<Utc>,
}

/// Returns a vector containing all the artifacts whose `artifact_id` start with
/// the given `search_term`.
pub async fn search_artifacts(
    search_term: impl AsRef<str>,
    database: &Pool<Postgres>,
) -> sqlx::Result<Vec<GeneralizedArtifactInfo>> {
    let search_term = format!("{}%", search_term.as_ref());
    let query_results = sqlx::query!(
        r#"
        select author, artifact_id, min(description) as description, max(creation_date) as latest_date, min(creation_date) as first_date, max(version) as latest_version
        from artifacts
        where artifact_id like $1 
        group by author, artifact_id
        "#,
        &search_term
    )
    .fetch_all(database)
    .await?;

    let artifacts: Vec<GeneralizedArtifactInfo> = query_results
        .into_iter()
        .map(|row| GeneralizedArtifactInfo {
            search_term: search_term.clone(),
            author: row.author,
            artifact_id: row.artifact_id,
            latest_version: row.latest_version.unwrap_or_default(),
            description: row.description.unwrap_or_default(),
            latest_date: row.latest_date.unwrap_or_default(),
            first_date: row.first_date.unwrap_or_default(),
        })
        .collect();

    Ok(artifacts)
}

/// Fetches all the versions of a given artifact spec. The version in the `spec` gets
/// ignored.
pub async fn all_artifact_versions(
    spec: &ArtifactSpec,
    database: &Pool<Postgres>,
) -> sqlx::Result<Vec<ArtifactsRow>> {
    let versions = sqlx::query!(
        r#"
        SELECT author, artifact_id, version, description, creation_date
        FROM artifacts
        WHERE author = $1 AND artifact_id = $2
        "#,
        spec.author,
        spec.artifact_id
    )
    .fetch_all(database)
    .await?;

    let versions: Vec<ArtifactsRow> = versions
        .into_iter()
        .map(|row| ArtifactsRow {
            spec: ArtifactSpec::new(row.author, row.artifact_id, row.version),
            description: row.description,
            creation_date: row.creation_date,
        })
        .collect();

    Ok(versions)
}

pub async fn first_latest_release_date(
    spec: &ArtifactSpec,
    database: &Pool<Postgres>,
) -> sqlx::Result<(chrono::DateTime<Utc>, chrono::DateTime<Utc>)> {
    log::info!("Spec: {spec}");
    let result = sqlx::query!(
        r#"
        SELECT 
            (SELECT creation_date
            FROM   artifacts
            WHERE  author = $1 
                   AND artifact_id = $2
            ORDER BY creation_date DESC LIMIT 1) AS first_date,
           (SELECT creation_date
            FROM   artifacts
            WHERE  author = $1 
                   AND artifact_id = $2
             ORDER BY creation_date LIMIT 1) AS last_date
    "#,
        spec.author,
        spec.artifact_id
    )
    .fetch_one(database)
    .await?;

    match (result.first_date, result.last_date) {
        (Some(first_date), Some(last_date)) => Ok((first_date, last_date)),
        _ => Err(sqlx::Error::ColumnNotFound(format!(
            "No dates for artifact {}_{}: {result:?}",
            spec.author, spec.artifact_id
        ))),
    }
}

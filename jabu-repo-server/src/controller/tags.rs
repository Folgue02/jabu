use jabu_config::model::ArtifactSpec;
use sqlx::{Pool, Postgres};

/// Returns a collection of all the **distinct** tags registered
/// to the artifact spec specified.
///
/// # NOTE
/// The `version` field of the artifact spec gets ignored.
pub async fn tags_of_artifact(
    spec: &ArtifactSpec,
    database: &Pool<Postgres>,
) -> sqlx::Result<Vec<String>> {
    let tag_rows = sqlx::query!(
        r#"
        SELECT DISTINCT tag
        FROM artifact_tags
        WHERE artifact_id = $1 AND author = $2
        "#,
        spec.artifact_id,
        spec.author
    )
    .fetch_all(database)
    .await?;
    Ok(tag_rows.into_iter().map(|row| row.tag).collect())
}

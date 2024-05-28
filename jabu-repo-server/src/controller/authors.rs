use sqlx::{Pool, Postgres, Row};

/// Checks if the uuid given matches the auther specified.
pub async fn check_uuid_author(
    author: impl Into<String>,
    uuid: impl Into<String>,
    database: &Pool<Postgres>,
) -> sqlx::Result<bool> {
    let query_result = sqlx::query(
        "SELECT COUNT(*) AS count
        FROM authors 
        WHERE author = $1 AND uuid_key = $2",
    )
    .bind(author.into())
    .bind(uuid.into())
    .fetch_one(database)
    .await?;

    Ok(query_result.get::<i64, _>("count") == 1)
}

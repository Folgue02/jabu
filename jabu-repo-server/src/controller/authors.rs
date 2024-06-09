use sqlx::{Pool, Postgres, Row};

/// Checks if the uuid given matches the auther specified.
pub async fn check_uuid_author(
    author: impl Into<String>,
    uuid: impl Into<String>,
    database: &Pool<Postgres>,
) -> sqlx::Result<bool> {
    let author = author.into();
    let uuid = uuid.into();
    let query_result = sqlx::query(
        "SELECT COUNT(*) AS existence
        FROM authors 
        WHERE author = $1 AND uuid_key = $2",
    )
    .bind(&author)
    .bind(&uuid)
    .fetch_one(database)
    .await?;

    let count = query_result.get::<i64, _>("existence");
    log::info!("Checking uuid {uuid} from author {author}");
    Ok(count == 1)
}

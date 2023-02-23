use sqlx::SqlitePool;

struct SqliteClient {
    pub(crate) pool: SqlitePool,
    // Keep on to url in case the connection is dropped unexplainably
    pub(crate) url: String,
}

impl SqliteClient {
    pub async fn new(url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(url).await?;
        sqlx::query(include_str!("./create_tables.sql")).execute(&pool).await?;
        Ok(SqliteClient { pool, url: url.to_string() })
    }

}

use crate::error::CustomError;
use sqlx::{migrate::MigrateDatabase, Sqlite};

pub type Pool = sqlx::Pool<Sqlite>;

pub async fn create_pool(db_url: &str) -> Result<Pool, CustomError> {
    Sqlite::create_database(db_url).await?;

    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await?;

    init_db(&pool).await?;

    Ok(pool)
}

pub async fn init_db(pool: &Pool) -> Result<(), CustomError> {
    sqlx::query_file!("src/db/init.sql").execute(pool).await?;
    Ok(())
}

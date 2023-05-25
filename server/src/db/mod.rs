#![allow(unused)]

use crate::error::CustomError;

type Pool = sqlx::Pool<sqlx::Sqlite>;

pub async fn create_pool(db_url: &str) -> Result<Pool, CustomError> {
    sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await
        .map_err(|_| CustomError::DatabaseError)
}

pub async fn init_db(pool: Pool) -> Result<(), CustomError> {
    sqlx::query_file!("src/db/init.sql")
        .execute(&pool)
        .await
        .map_err(|_| CustomError::DatabaseError)?;

    Ok(())
}

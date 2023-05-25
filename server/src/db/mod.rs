#![allow(unused)]

use sqlx::{migrate::MigrateDatabase, Executor, Sqlite};

use crate::error::CustomError;

pub type Pool = sqlx::Pool<Sqlite>;

pub async fn create_pool(db_url: &str) -> Result<Pool, CustomError> {
    Sqlite::create_database(db_url)
        .await
        .map_err(CustomError::DatabaseError);

    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await
        .map_err(CustomError::DatabaseError)?;

    init_db(&pool).await?;

    Ok(pool)
}

pub async fn init_db(pool: &Pool) -> Result<(), CustomError> {
    pool.execute(include_str!("init.sql"))
        .await
        .map_err(CustomError::DatabaseError)?;

    Ok(())
}

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ArticlePreview {
    pub id: u32,
    pub title: String,
    pub date: NaiveDate,
}

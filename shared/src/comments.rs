use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: Option<u32>,
    pub user_id: u32,
    pub article_id: u32,
    pub content: String,
    pub date: Option<chrono::NaiveDate>,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: Option<u32>,
    pub title: String,
    pub content: String,
    pub date: chrono::NaiveDate,
}

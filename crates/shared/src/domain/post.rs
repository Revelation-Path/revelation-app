use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "db", derive(sqlx::Type))]
#[cfg_attr(
    feature = "db",
    sqlx(type_name = "post_type", rename_all = "snake_case")
)]
pub enum PostType {
    Sermon,     // Проповедь
    Discussion, // Обсуждение
    Testimony,  // Свидетельство
    Prayer,     // Молитвенная нужда
    Event       // Событие
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id:         Uuid,
    pub author_id:  Uuid,
    pub church_id:  Option<Uuid>, // None = общая лента
    pub post_type:  PostType,
    pub title:      String,
    pub content:    String,
    pub media_urls: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreatePost {
    pub church_id: Option<Uuid>,
    pub post_type: PostType,

    #[validate(length(min = 1, max = 300))]
    pub title: String,

    #[validate(length(min = 1, max = 50000))]
    pub content: String,

    #[validate(length(max = 10))]
    pub media_urls: Vec<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostComment {
    pub id:         Uuid,
    pub post_id:    Uuid,
    pub author_id:  Uuid,
    pub content:    String,
    pub created_at: DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateComment {
    #[validate(length(min = 1, max = 5000))]
    pub content: String
}

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "db", derive(sqlx::Type))]
#[cfg_attr(
    feature = "db",
    sqlx(type_name = "testament", rename_all = "snake_case")
)]
pub enum Testament {
    Old,
    New
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub id:             i16,
    pub name:           String,
    pub name_ru:        String,
    pub abbreviation:   String,
    pub testament:      Testament,
    pub chapters_count: i16
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verse {
    pub id:      i32,
    pub book_id: i16,
    pub chapter: i16,
    pub verse:   i16,
    pub text:    String
}

/// Стих дня для программы чтения Библии за год
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyReading {
    pub id:          Uuid,
    pub day_of_year: i16, // 1-365
    pub date:        NaiveDate,
    pub verses:      Vec<Verse>
}

/// Ответ пользователя на стих дня
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerseResponse {
    pub id:               Uuid,
    pub user_id:          Uuid,
    pub daily_reading_id: Uuid,
    pub content:          String,
    pub created_at:       DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateVerseResponse {
    pub daily_reading_id: Uuid,

    #[validate(length(min = 1, max = 10000))]
    pub content: String
}

/// Результат поиска по Библии (симфония)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub verse:      Verse,
    pub book_name:  String,
    pub highlights: Vec<(usize, usize)> // позиции найденных слов
}

/// Периокопа - заголовок раздела Библии
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pericope {
    pub chapter: i16,
    pub verse:   i16,
    pub heading: String
}

/// Информация о главе
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterInfo {
    pub chapter:     i16,
    pub verse_count: i16
}

use gloo_net::http::Request;
use shared::{Book, ChapterInfo, DailyReading, Pericope, SearchResult, Testament, User, Verse};
use uuid::Uuid;

const API_BASE: &str = "/api";

pub async fn get_or_create_user(user_id: Uuid) -> Result<User, String> {
    // Try to get user first
    let response = Request::get(&format!("{}/users/{}", API_BASE, user_id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.ok() {
        return response.json().await.map_err(|e| e.to_string());
    }

    // Create user if not exists
    let response = Request::post(&format!("{}/users", API_BASE))
        .json(&serde_json::json!({ "id": user_id }))
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

pub async fn get_books() -> Result<Vec<Book>, String> {
    let response = Request::get(&format!("{}/bible/books", API_BASE))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

pub async fn get_books_by_testament(testament: Testament) -> Result<Vec<Book>, String> {
    let testament_str = match testament {
        Testament::Old => "old",
        Testament::New => "new"
    };
    let response = Request::get(&format!(
        "{}/bible/books?testament={}",
        API_BASE, testament_str
    ))
    .send()
    .await
    .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

pub async fn get_chapter(book_id: i16, chapter: i16) -> Result<Vec<Verse>, String> {
    let response = Request::get(&format!(
        "{}/bible/books/{}/chapters/{}",
        API_BASE, book_id, chapter
    ))
    .send()
    .await
    .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

pub async fn get_pericopes(book_id: i16) -> Result<Vec<Pericope>, String> {
    let response = Request::get(&format!("{}/bible/books/{}/pericopes", API_BASE, book_id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

pub async fn get_chapters_info(book_id: i16) -> Result<Vec<ChapterInfo>, String> {
    let response = Request::get(&format!(
        "{}/bible/books/{}/chapters-info",
        API_BASE, book_id
    ))
    .send()
    .await
    .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

pub async fn search_bible(query: &str) -> Result<Vec<SearchResult>, String> {
    let response = Request::get(&format!("{}/bible/search?q={}", API_BASE, query))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

pub async fn get_symphony(word: &str) -> Result<SymphonyResponse, String> {
    let response = Request::get(&format!("{}/bible/symphony/{}", API_BASE, word))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

#[derive(serde::Deserialize)]
pub struct SymphonyResponse {
    pub word:        String,
    pub total_count: i64,
    pub verses:      Vec<SearchResult>
}

pub async fn get_today_reading() -> Result<Option<DailyReading>, String> {
    let response = Request::get(&format!("{}/bible/today", API_BASE))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

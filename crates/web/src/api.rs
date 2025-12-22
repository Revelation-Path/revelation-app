use gloo_net::http::Request;
use shared::{
    Book, ChapterInfo, DailyReading, Pericope, SearchResult, Song, SongSearchResult, SongSummary,
    Songbook, SongbookEdition, Testament, User, Verse
};
use uuid::Uuid;

use crate::bible::BibleProvider;

fn api_base() -> String {
    let host = web_sys::window()
        .and_then(|w| w.location().host().ok())
        .unwrap_or_default();

    if host.contains("revelation-path.ru") {
        "https://api.revelation-path.ru/api".to_string()
    } else {
        "/api".to_string()
    }
}

/// Get books - from S3 cache, fallback to API
pub async fn get_books_cached() -> Result<Vec<Book>, String> {
    match BibleProvider::init().await {
        Ok(cache) => Ok(cache.get_books()),
        Err(_) => get_books().await
    }
}

/// Get chapter - from S3 cache, fallback to API
pub async fn get_chapter_cached(book_id: i16, chapter: i16) -> Result<Vec<Verse>, String> {
    match BibleProvider::init().await {
        Ok(cache) => cache
            .get_chapter(book_id, chapter)
            .ok_or_else(|| "Chapter not found".to_string()),
        Err(_) => get_chapter(book_id, chapter).await
    }
}

pub async fn get_or_create_user(user_id: Uuid) -> Result<User, String> {
    // Try to get user first
    let response = Request::get(&format!("{}/users/{}", api_base(), user_id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.ok() {
        return response.json().await.map_err(|e| e.to_string());
    }

    // Create user if not exists
    let response = Request::post(&format!("{}/users", api_base()))
        .json(&serde_json::json!({ "id": user_id }))
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

pub async fn get_books() -> Result<Vec<Book>, String> {
    let response = Request::get(&format!("{}/bible/books", api_base()))
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
        api_base(),
        testament_str
    ))
    .send()
    .await
    .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

pub async fn get_chapter(book_id: i16, chapter: i16) -> Result<Vec<Verse>, String> {
    let response = Request::get(&format!(
        "{}/bible/books/{}/chapters/{}",
        api_base(),
        book_id,
        chapter
    ))
    .send()
    .await
    .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

pub async fn get_pericopes(book_id: i16) -> Result<Vec<Pericope>, String> {
    let response = Request::get(&format!("{}/bible/books/{}/pericopes", api_base(), book_id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

pub async fn get_chapters_info(book_id: i16) -> Result<Vec<ChapterInfo>, String> {
    let response = Request::get(&format!(
        "{}/bible/books/{}/chapters-info",
        api_base(),
        book_id
    ))
    .send()
    .await
    .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

pub async fn search_bible(query: &str) -> Result<Vec<SearchResult>, String> {
    let response = Request::get(&format!("{}/bible/search?q={}", api_base(), query))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

pub async fn get_symphony(word: &str) -> Result<SymphonyResponse, String> {
    let response = Request::get(&format!("{}/bible/symphony/{}", api_base(), word))
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
    let response = Request::get(&format!("{}/bible/today", api_base()))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

// ========== Songs API ==========

/// Get all songbooks
pub async fn get_songbooks() -> Result<Vec<Songbook>, String> {
    let response = Request::get(&format!("{}/songs/songbooks", api_base()))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// Get a single songbook by ID
pub async fn get_songbook(id: Uuid) -> Result<Songbook, String> {
    let response = Request::get(&format!("{}/songs/songbooks/{}", api_base(), id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// Get songbook editions
pub async fn get_songbook_editions(songbook_id: Uuid) -> Result<Vec<SongbookEdition>, String> {
    let response = Request::get(&format!(
        "{}/songs/songbooks/{}/editions",
        api_base(),
        songbook_id
    ))
    .send()
    .await
    .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// Get songs from a specific songbook
pub async fn get_songs_by_songbook(
    songbook_id: Uuid,
    page: Option<u32>,
    limit: Option<u32>
) -> Result<Vec<SongSummary>, String> {
    let limit = limit.unwrap_or(50) as i64;
    let offset = page.map(|p| ((p.max(1) - 1) as i64) * limit).unwrap_or(0);

    let response = Request::get(&format!(
        "{}/songs?songbook_id={}&limit={}&offset={}",
        api_base(),
        songbook_id,
        limit,
        offset
    ))
    .send()
    .await
    .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// Get a single song by ID
pub async fn get_song(song_id: Uuid) -> Result<Song, String> {
    let response = Request::get(&format!("{}/songs/{}", api_base(), song_id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// Search songs
pub async fn search_songs(query: &str) -> Result<Vec<SongSearchResult>, String> {
    let response = Request::get(&format!("{}/songs/search?q={}", api_base(), query))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// Get transposed song content
pub async fn transpose_song(song_id: Uuid, semitones: i32) -> Result<Song, String> {
    let response = Request::get(&format!(
        "{}/songs/{}/transpose/{}",
        api_base(),
        song_id,
        semitones
    ))
    .send()
    .await
    .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

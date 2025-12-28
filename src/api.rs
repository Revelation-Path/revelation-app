//! API client for Revelation backend services.

use gloo_net::http::Request;
use revelation_bible::{
    Book, ChapterInfo, DailyReading, Pericope, SearchResult, Testament, Verse,
};
use revelation_songbook::{Song, SongSearchResult, SongSummary, Songbook, SongbookEdition};
use revelation_user::RUser;
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

/// Fetches Bible books from S3 cache with API fallback.
///
/// # Errors
///
/// Returns error string if both cache and API requests fail.
pub async fn get_books_cached() -> Result<Vec<Book>, String> {
    match BibleProvider::init().await {
        Ok(cache) => Ok(cache.get_books()),
        Err(_) => get_books().await,
    }
}

/// Fetches chapter verses from S3 cache with API fallback.
///
/// # Errors
///
/// Returns error string if chapter not found or request fails.
pub async fn get_chapter_cached(book_id: i16, chapter: i16) -> Result<Vec<Verse>, String> {
    match BibleProvider::init().await {
        Ok(cache) => cache
            .get_chapter(book_id, chapter)
            .ok_or_else(|| "Chapter not found".to_string()),
        Err(_) => get_chapter(book_id, chapter).await,
    }
}

/// Gets or creates a user by ID.
///
/// # Errors
///
/// Returns error string if network request or JSON parsing fails.
pub async fn get_or_create_user(user_id: Uuid) -> Result<RUser, String> {
    let url = format!("{}/users/{user_id}", api_base());
    let response = Request::get(&url).send().await.map_err(|e| e.to_string())?;

    if response.ok() {
        return response.json().await.map_err(|e| e.to_string());
    }

    let url = format!("{}/users", api_base());
    let response = Request::post(&url)
        .json(&serde_json::json!({ "id": user_id }))
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// Fetches all Bible books.
///
/// # Errors
///
/// Returns error string if network request or JSON parsing fails.
pub async fn get_books() -> Result<Vec<Book>, String> {
    let url = format!("{}/bible/books", api_base());
    let response = Request::get(&url).send().await.map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// Fetches Bible books filtered by testament.
///
/// # Errors
///
/// Returns error string if network request or JSON parsing fails.
pub async fn get_books_by_testament(testament: Testament) -> Result<Vec<Book>, String> {
    let testament_str = match testament {
        Testament::Old => "old",
        Testament::New => "new",
    };
    let url = format!("{}/bible/books?testament={testament_str}", api_base());
    let response = Request::get(&url).send().await.map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// Fetches verses for a specific chapter.
///
/// # Errors
///
/// Returns error string if network request or JSON parsing fails.
pub async fn get_chapter(book_id: i16, chapter: i16) -> Result<Vec<Verse>, String> {
    let url = format!("{}/bible/books/{book_id}/chapters/{chapter}", api_base());
    let response = Request::get(&url).send().await.map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// Fetches pericopes for a book.
///
/// # Errors
///
/// Returns error string if network request or JSON parsing fails.
pub async fn get_pericopes(book_id: i16) -> Result<Vec<Pericope>, String> {
    let url = format!("{}/bible/books/{book_id}/pericopes", api_base());
    let response = Request::get(&url).send().await.map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// Fetches chapter metadata for a book.
///
/// # Errors
///
/// Returns error string if network request or JSON parsing fails.
pub async fn get_chapters_info(book_id: i16) -> Result<Vec<ChapterInfo>, String> {
    let url = format!("{}/bible/books/{book_id}/chapters-info", api_base());
    let response = Request::get(&url).send().await.map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// Searches Bible text.
///
/// # Errors
///
/// Returns error string if network request or JSON parsing fails.
pub async fn search_bible(query: &str) -> Result<Vec<SearchResult>, String> {
    let url = format!("{}/bible/search?q={query}", api_base());
    let response = Request::get(&url).send().await.map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// Fetches symphony (concordance) data for a word.
///
/// # Errors
///
/// Returns error string if network request or JSON parsing fails.
pub async fn get_symphony(word: &str) -> Result<SymphonyResponse, String> {
    let url = format!("{}/bible/symphony/{word}", api_base());
    let response = Request::get(&url).send().await.map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// Symphony API response.
#[derive(serde::Deserialize)]
pub struct SymphonyResponse {
    /// The searched word.
    pub word: String,
    /// Total occurrences count.
    pub total_count: i64,
    /// Matching verses.
    pub verses: Vec<SearchResult>,
}

/// Fetches today's Bible reading.
///
/// # Errors
///
/// Returns error string if network request or JSON parsing fails.
pub async fn get_today_reading() -> Result<Option<DailyReading>, String> {
    let url = format!("{}/bible/today", api_base());
    let response = Request::get(&url).send().await.map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

// ========== Songs API ==========

/// Fetches all songbooks.
///
/// # Errors
///
/// Returns error string if network request or JSON parsing fails.
pub async fn get_songbooks() -> Result<Vec<Songbook>, String> {
    let url = format!("{}/songs/songbooks", api_base());
    let response = Request::get(&url).send().await.map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// Fetches a songbook by ID.
///
/// # Errors
///
/// Returns error string if network request or JSON parsing fails.
pub async fn get_songbook(id: Uuid) -> Result<Songbook, String> {
    let url = format!("{}/songs/songbooks/{id}", api_base());
    let response = Request::get(&url).send().await.map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// Fetches editions for a songbook.
///
/// # Errors
///
/// Returns error string if network request or JSON parsing fails.
pub async fn get_songbook_editions(songbook_id: Uuid) -> Result<Vec<SongbookEdition>, String> {
    let url = format!("{}/songs/songbooks/{songbook_id}/editions", api_base());
    let response = Request::get(&url).send().await.map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// Fetches songs from a songbook with pagination.
///
/// # Errors
///
/// Returns error string if network request or JSON parsing fails.
pub async fn get_songs_by_songbook(
    songbook_id: Uuid,
    page: Option<u32>,
    limit: Option<u32>,
) -> Result<Vec<SongSummary>, String> {
    let limit = i64::from(limit.unwrap_or(50));
    let offset = page.map_or(0, |p| i64::from(p.max(1) - 1) * limit);

    let url = format!(
        "{}/songs?songbook_id={songbook_id}&limit={limit}&offset={offset}",
        api_base()
    );
    let response = Request::get(&url).send().await.map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// Fetches a song by ID.
///
/// # Errors
///
/// Returns error string if network request or JSON parsing fails.
pub async fn get_song(song_id: Uuid) -> Result<Song, String> {
    let url = format!("{}/songs/{song_id}", api_base());
    let response = Request::get(&url).send().await.map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// Searches songs by query.
///
/// # Errors
///
/// Returns error string if network request or JSON parsing fails.
pub async fn search_songs(query: &str) -> Result<Vec<SongSearchResult>, String> {
    let url = format!("{}/songs/search?q={query}", api_base());
    let response = Request::get(&url).send().await.map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// Fetches transposed song content.
///
/// # Errors
///
/// Returns error string if network request or JSON parsing fails.
pub async fn transpose_song(song_id: Uuid, semitones: i32) -> Result<Song, String> {
    let url = format!("{}/songs/{song_id}/transpose/{semitones}", api_base());
    let response = Request::get(&url).send().await.map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

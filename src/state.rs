//! Application state management.

use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use revelation_user::RUser;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;

use crate::bible::{BibleCache, BibleProvider};

const USER_ID_KEY: &str = "revelation_user_id";
const BIBLE_BOOK_KEY: &str = "bible_current_book";
const BIBLE_CHAPTER_KEY: &str = "bible_current_chapter";
const ONLY_WITH_CHORDS_KEY: &str = "songs_only_with_chords";

/// Global application state.
#[derive(Clone)]
pub struct AppState {
    /// Current user ID.
    pub user_id: RwSignal<Uuid>,
    /// Current user data.
    pub user: RwSignal<Option<RUser>>,
    /// Loading state indicator.
    pub is_loading: RwSignal<bool>,
    /// Sidebar collapsed state.
    pub sidebar_collapsed: RwSignal<bool>,
    /// Current Bible book ID.
    pub current_book: RwSignal<i16>,
    /// Current Bible chapter number.
    pub current_chapter: RwSignal<i16>,
    /// Cached Bible data.
    pub bible: RwSignal<Option<BibleCache>>,
    /// Filter for songs with chords only.
    pub only_with_chords: RwSignal<bool>,
}

impl AppState {
    /// Initializes application state from localStorage.
    #[must_use]
    pub fn init() -> Self {
        let user_id = LocalStorage::get::<String>(USER_ID_KEY)
            .ok()
            .and_then(|s| Uuid::parse_str(&s).ok())
            .unwrap_or_else(|| {
                let id = Uuid::now_v7();
                let _ = LocalStorage::set(USER_ID_KEY, id.to_string());
                id
            });

        let saved_book = LocalStorage::get::<i16>(BIBLE_BOOK_KEY).unwrap_or(1);
        let saved_chapter = LocalStorage::get::<i16>(BIBLE_CHAPTER_KEY).unwrap_or(1);
        let saved_chords_filter = LocalStorage::get::<bool>(ONLY_WITH_CHORDS_KEY).unwrap_or(false);

        let bible = RwSignal::new(None);

        let bible_signal = bible;
        spawn_local(async move {
            match BibleProvider::init().await {
                Ok(cache) => bible_signal.set(Some(cache)),
                Err(e) => {
                    web_sys::console::error_1(&format!("Failed to load Bible: {e}").into());
                }
            }
        });

        let current_book = RwSignal::new(saved_book);
        let current_chapter = RwSignal::new(saved_chapter);
        let only_with_chords = RwSignal::new(saved_chords_filter);

        Effect::new(move |_| {
            let book = current_book.get();
            let chapter = current_chapter.get();
            let _ = LocalStorage::set(BIBLE_BOOK_KEY, book);
            let _ = LocalStorage::set(BIBLE_CHAPTER_KEY, chapter);
        });

        Effect::new(move |_| {
            let filter = only_with_chords.get();
            let _ = LocalStorage::set(ONLY_WITH_CHORDS_KEY, filter);
        });

        Self {
            user_id: RwSignal::new(user_id),
            user: RwSignal::new(None),
            is_loading: RwSignal::new(true),
            sidebar_collapsed: RwSignal::new(false),
            current_book,
            current_chapter,
            bible,
            only_with_chords,
        }
    }

    /// Returns the current user ID.
    #[must_use]
    pub fn user_id(&self) -> Uuid {
        self.user_id.get()
    }

    /// Returns chapter verses from cached Bible.
    #[must_use]
    pub fn get_chapter(&self, book_id: i16, chapter: i16) -> Option<Vec<revelation_bible::Verse>> {
        self.bible.get()?.get_chapter(book_id, chapter)
    }

    /// Returns all books from cached Bible.
    #[must_use]
    pub fn get_books(&self) -> Option<Vec<revelation_bible::Book>> {
        Some(self.bible.get()?.get_books())
    }
}

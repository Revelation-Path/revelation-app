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

#[derive(Clone)]
pub struct AppState {
    pub user_id:           RwSignal<Uuid>,
    pub user:              RwSignal<Option<RUser>>,
    pub is_loading:        RwSignal<bool>,
    pub sidebar_collapsed: RwSignal<bool>,
    pub current_book:      RwSignal<i16>,
    pub current_chapter:   RwSignal<i16>,
    pub bible:             RwSignal<Option<BibleCache>>,
    pub only_with_chords:  RwSignal<bool>
}

impl AppState {
    pub fn init() -> Self {
        // Get or create user ID from localStorage
        let user_id = LocalStorage::get::<String>(USER_ID_KEY)
            .ok()
            .and_then(|s| Uuid::parse_str(&s).ok())
            .unwrap_or_else(|| {
                let id = Uuid::now_v7();
                let _ = LocalStorage::set(USER_ID_KEY, id.to_string());
                id
            });

        // Load saved Bible position from localStorage
        let saved_book = LocalStorage::get::<i16>(BIBLE_BOOK_KEY).unwrap_or(1);
        let saved_chapter = LocalStorage::get::<i16>(BIBLE_CHAPTER_KEY).unwrap_or(1);
        let saved_chords_filter = LocalStorage::get::<bool>(ONLY_WITH_CHORDS_KEY).unwrap_or(false);

        let bible = RwSignal::new(None);

        // Load Bible from S3/cache asynchronously
        let bible_signal = bible;
        spawn_local(async move {
            match BibleProvider::init().await {
                Ok(cache) => bible_signal.set(Some(cache)),
                Err(e) => web_sys::console::error_1(&format!("Failed to load Bible: {}", e).into())
            }
        });

        let current_book = RwSignal::new(saved_book);
        let current_chapter = RwSignal::new(saved_chapter);
        let only_with_chords = RwSignal::new(saved_chords_filter);

        // Save Bible position to localStorage when changed
        Effect::new(move |_| {
            let book = current_book.get();
            let chapter = current_chapter.get();
            let _ = LocalStorage::set(BIBLE_BOOK_KEY, book);
            let _ = LocalStorage::set(BIBLE_CHAPTER_KEY, chapter);
        });

        // Save chords filter to localStorage when changed
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
            only_with_chords
        }
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id.get()
    }

    /// Get chapter from cached Bible
    pub fn get_chapter(
        &self,
        book_id: i16,
        chapter: i16
    ) -> Option<Vec<revelation_bible::Verse>> {
        self.bible.get()?.get_chapter(book_id, chapter)
    }

    /// Get all books from cached Bible
    pub fn get_books(&self) -> Option<Vec<revelation_bible::Book>> {
        Some(self.bible.get()?.get_books())
    }
}

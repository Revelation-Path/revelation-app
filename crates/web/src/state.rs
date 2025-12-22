use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use shared::User;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;

use crate::bible::{BibleCache, BibleProvider};

const USER_ID_KEY: &str = "revelation_user_id";

#[derive(Clone)]
pub struct AppState {
    pub user_id:           RwSignal<Uuid>,
    pub user:              RwSignal<Option<User>>,
    pub is_loading:        RwSignal<bool>,
    pub sidebar_collapsed: RwSignal<bool>,
    pub current_book:      RwSignal<i16>,
    pub current_chapter:   RwSignal<i16>,
    pub bible:             RwSignal<Option<BibleCache>>
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

        let bible = RwSignal::new(None);

        // Load Bible from S3/cache asynchronously
        let bible_signal = bible;
        spawn_local(async move {
            match BibleProvider::init().await {
                Ok(cache) => bible_signal.set(Some(cache)),
                Err(e) => web_sys::console::error_1(&format!("Failed to load Bible: {}", e).into())
            }
        });

        Self {
            user_id: RwSignal::new(user_id),
            user: RwSignal::new(None),
            is_loading: RwSignal::new(true),
            sidebar_collapsed: RwSignal::new(false),
            current_book: RwSignal::new(1),
            current_chapter: RwSignal::new(1),
            bible
        }
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id.get()
    }

    /// Get chapter from cached Bible
    pub fn get_chapter(&self, book_id: i16, chapter: i16) -> Option<Vec<shared::Verse>> {
        self.bible.get()?.get_chapter(book_id, chapter)
    }

    /// Get all books from cached Bible
    pub fn get_books(&self) -> Option<Vec<shared::Book>> {
        Some(self.bible.get()?.get_books())
    }
}

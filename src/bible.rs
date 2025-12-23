use std::collections::HashMap;

use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use revelation_bible::{Book, Testament, Verse};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

#[cfg(debug_assertions)]
const BIBLE_URL: &str = "/bible/synodal.json";

#[cfg(not(debug_assertions))]
const BIBLE_URL: &str = "https://s3.twcstorage.ru/7f594bdf-revelation/synodal.json";
const CACHE_KEY: &str = "bible_synodal";
const CACHE_VERSION_KEY: &str = "bible_version";
const CURRENT_VERSION: &str = "1.0.0";

/// Raw Bible data from S3 JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawBook {
    pub abbrev:   String,
    pub chapters: Vec<Vec<String>>
}

/// Cached Bible with indexed access
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BibleCache {
    version:      String,
    books:        Vec<RawBook>,
    #[serde(skip)]
    abbrev_to_id: HashMap<String, i16>,
    #[serde(skip)]
    id_to_abbrev: HashMap<i16, String>
}

impl BibleCache {
    /// Initialize index maps after deserialization
    pub fn init_indices(&mut self) {
        let abbrev_order = [
            "gn", "ex", "lv", "nm", "dt", "js", "jud", "rt", "1sm", "2sm", "1kgs", "2kgs", "1ch",
            "2ch", "ezr", "ne", "et", "job", "ps", "prv", "ec", "so", "is", "jr", "lm", "ez",
            "dn", "ho", "jl", "am", "ob", "jn", "mc", "na", "hk", "zp", "hg", "zc", "ml", "mt",
            "mk", "lk", "jo", "act", "rm", "1co", "2co", "gl", "eph", "ph", "cl", "1ts", "2ts",
            "1tm", "2tm", "tt", "phm", "hb", "jm", "1pe", "2pe", "1jo", "2jo", "3jo", "jd", "re"
        ];

        for (idx, abbrev) in abbrev_order.iter().enumerate() {
            let id = (idx + 1) as i16;
            self.abbrev_to_id.insert(abbrev.to_string(), id);
            self.id_to_abbrev.insert(id, abbrev.to_string());
        }
    }

    /// Get book by ID (1-66)
    pub fn get_book(&self, book_id: i16) -> Option<&RawBook> {
        let abbrev = self.id_to_abbrev.get(&book_id)?;
        self.books.iter().find(|b| &b.abbrev == abbrev)
    }

    /// Get chapter verses
    pub fn get_chapter(&self, book_id: i16, chapter: i16) -> Option<Vec<Verse>> {
        let book = self.get_book(book_id)?;
        let chapter_idx = (chapter - 1) as usize;
        let verses_text = book.chapters.get(chapter_idx)?;

        Some(
            verses_text
                .iter()
                .enumerate()
                .map(|(idx, text)| Verse {
                    id: 0,
                    book_id,
                    chapter,
                    verse: (idx + 1) as i16,
                    text: text.clone()
                })
                .collect()
        )
    }

    /// Get all books with metadata
    pub fn get_books(&self) -> Vec<Book> {
        let book_names_ru = [
            "Бытие",
            "Исход",
            "Левит",
            "Числа",
            "Второзаконие",
            "Иисус Навин",
            "Судей",
            "Руфь",
            "1 Царств",
            "2 Царств",
            "3 Царств",
            "4 Царств",
            "1 Паралипоменон",
            "2 Паралипоменон",
            "Ездра",
            "Неемия",
            "Есфирь",
            "Иов",
            "Псалтирь",
            "Притчи",
            "Екклесиаст",
            "Песнь Песней",
            "Исаия",
            "Иеремия",
            "Плач Иеремии",
            "Иезекииль",
            "Даниил",
            "Осия",
            "Иоиль",
            "Амос",
            "Авдий",
            "Иона",
            "Михей",
            "Наум",
            "Аввакум",
            "Софония",
            "Аггей",
            "Захария",
            "Малахия",
            "От Матфея",
            "От Марка",
            "От Луки",
            "От Иоанна",
            "Деяния",
            "Римлянам",
            "1 Коринфянам",
            "2 Коринфянам",
            "Галатам",
            "Ефесянам",
            "Филиппийцам",
            "Колоссянам",
            "1 Фессалоникийцам",
            "2 Фессалоникийцам",
            "1 Тимофею",
            "2 Тимофею",
            "Титу",
            "Филимону",
            "Евреям",
            "Иакова",
            "1 Петра",
            "2 Петра",
            "1 Иоанна",
            "2 Иоанна",
            "3 Иоанна",
            "Иуды",
            "Откровение"
        ];

        let abbreviations = [
            "Быт",
            "Исх",
            "Лев",
            "Чис",
            "Втор",
            "Нав",
            "Суд",
            "Руф",
            "1Цар",
            "2Цар",
            "3Цар",
            "4Цар",
            "1Пар",
            "2Пар",
            "Езд",
            "Неем",
            "Есф",
            "Иов",
            "Пс",
            "Притч",
            "Еккл",
            "Песн",
            "Ис",
            "Иер",
            "Плач",
            "Иез",
            "Дан",
            "Ос",
            "Иоил",
            "Ам",
            "Авд",
            "Ион",
            "Мих",
            "Наум",
            "Авв",
            "Соф",
            "Агг",
            "Зах",
            "Мал",
            "Мф",
            "Мк",
            "Лк",
            "Ин",
            "Деян",
            "Рим",
            "1Кор",
            "2Кор",
            "Гал",
            "Еф",
            "Флп",
            "Кол",
            "1Фес",
            "2Фес",
            "1Тим",
            "2Тим",
            "Тит",
            "Флм",
            "Евр",
            "Иак",
            "1Пет",
            "2Пет",
            "1Ин",
            "2Ин",
            "3Ин",
            "Иуд",
            "Откр"
        ];

        self.books
            .iter()
            .enumerate()
            .map(|(idx, raw_book)| {
                let id = (idx + 1) as i16;
                let testament = if id <= 39 {
                    Testament::Old
                } else {
                    Testament::New
                };

                Book {
                    id,
                    name: raw_book.abbrev.clone(),
                    name_ru: book_names_ru.get(idx).unwrap_or(&"").to_string(),
                    abbreviation: abbreviations.get(idx).unwrap_or(&"").to_string(),
                    testament,
                    chapters_count: raw_book.chapters.len() as i16
                }
            })
            .collect()
    }
}

/// Bible data provider with caching
pub struct BibleProvider;

impl BibleProvider {
    /// Check if cache is valid
    fn is_cache_valid() -> bool {
        LocalStorage::get::<String>(CACHE_VERSION_KEY)
            .map(|v| v == CURRENT_VERSION)
            .unwrap_or(false)
    }

    /// Load from LocalStorage
    fn load_from_storage() -> Option<BibleCache> {
        if !Self::is_cache_valid() {
            return None;
        }

        LocalStorage::get::<BibleCache>(CACHE_KEY)
            .ok()
            .map(|mut cache| {
                cache.init_indices();
                cache
            })
    }

    /// Save to LocalStorage
    fn save_to_storage(cache: &BibleCache) {
        let _ = LocalStorage::set(CACHE_KEY, cache);
        let _ = LocalStorage::set(CACHE_VERSION_KEY, CURRENT_VERSION);
    }

    /// Fetch Bible data (local in dev, S3 in prod)
    pub async fn fetch_bible() -> Result<BibleCache, String> {
        Self::fetch_from_url(BIBLE_URL).await
    }

    /// Fetch from a specific URL
    async fn fetch_from_url(url: &str) -> Result<BibleCache, String> {
        let response = Request::get(url)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if !response.ok() {
            return Err(format!("HTTP error: {}", response.status()));
        }

        let books: Vec<RawBook> = response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))?;

        let mut cache = BibleCache {
            version: CURRENT_VERSION.to_string(),
            books,
            ..Default::default()
        };
        cache.init_indices();

        Ok(cache)
    }

    /// Initialize provider - load from cache or fetch
    pub async fn init() -> Result<BibleCache, String> {
        // Try LocalStorage first
        if let Some(cache) = Self::load_from_storage() {
            return Ok(cache);
        }

        // Fetch Bible
        let cache = Self::fetch_bible().await?;
        Self::save_to_storage(&cache);

        Ok(cache)
    }

    /// Prefetch Bible data in background (call on app start)
    pub fn prefetch() {
        spawn_local(async {
            if Self::load_from_storage().is_none()
                && let Ok(cache) = Self::fetch_bible().await
            {
                Self::save_to_storage(&cache);
                web_sys::console::log_1(&"Bible prefetched and cached".into());
            }
        });
    }
}

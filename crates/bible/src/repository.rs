use shared::{Book, ChapterInfo, Pericope, Testament, Verse};
use sqlx::PgPool;

pub struct BibleRepository {
    pool: PgPool
}

impl BibleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    }

    /// Get all books
    pub async fn get_books(&self) -> Result<Vec<Book>, sqlx::Error> {
        sqlx::query_as!(
            Book,
            r#"
            SELECT
                id,
                name,
                name_ru,
                abbreviation,
                testament as "testament: Testament",
                chapters_count
            FROM bible_books
            ORDER BY id
            "#
        )
        .fetch_all(&self.pool)
        .await
    }

    /// Get books by testament
    pub async fn get_books_by_testament(
        &self,
        testament: Testament
    ) -> Result<Vec<Book>, sqlx::Error> {
        sqlx::query_as!(
            Book,
            r#"
            SELECT
                id,
                name,
                name_ru,
                abbreviation,
                testament as "testament: Testament",
                chapters_count
            FROM bible_books
            WHERE testament = $1
            ORDER BY id
            "#,
            testament as Testament
        )
        .fetch_all(&self.pool)
        .await
    }

    /// Get book by id
    pub async fn get_book(&self, id: i16) -> Result<Option<Book>, sqlx::Error> {
        sqlx::query_as!(
            Book,
            r#"
            SELECT
                id,
                name,
                name_ru,
                abbreviation,
                testament as "testament: Testament",
                chapters_count
            FROM bible_books
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    /// Get verses for a chapter
    pub async fn get_chapter(
        &self,
        book_id: i16,
        chapter: i16
    ) -> Result<Vec<Verse>, sqlx::Error> {
        sqlx::query_as!(
            Verse,
            r#"
            SELECT id, book_id, chapter, verse, text
            FROM bible_verses
            WHERE book_id = $1 AND chapter = $2
            ORDER BY verse
            "#,
            book_id,
            chapter
        )
        .fetch_all(&self.pool)
        .await
    }

    /// Get specific verse
    pub async fn get_verse(
        &self,
        book_id: i16,
        chapter: i16,
        verse: i16
    ) -> Result<Option<Verse>, sqlx::Error> {
        sqlx::query_as!(
            Verse,
            r#"
            SELECT id, book_id, chapter, verse, text
            FROM bible_verses
            WHERE book_id = $1 AND chapter = $2 AND verse = $3
            "#,
            book_id,
            chapter,
            verse
        )
        .fetch_optional(&self.pool)
        .await
    }

    /// Get range of verses
    pub async fn get_verses_range(
        &self,
        book_id: i16,
        chapter: i16,
        start_verse: i16,
        end_verse: i16
    ) -> Result<Vec<Verse>, sqlx::Error> {
        sqlx::query_as!(
            Verse,
            r#"
            SELECT id, book_id, chapter, verse, text
            FROM bible_verses
            WHERE book_id = $1 AND chapter = $2 AND verse >= $3 AND verse <= $4
            ORDER BY verse
            "#,
            book_id,
            chapter,
            start_verse,
            end_verse
        )
        .fetch_all(&self.pool)
        .await
    }

    /// Get pericopes for a book
    pub async fn get_pericopes(&self, book_id: i16) -> Result<Vec<Pericope>, sqlx::Error> {
        sqlx::query_as!(
            Pericope,
            r#"
            SELECT chapter, verse, heading
            FROM bible_pericopes
            WHERE book_id = $1
            ORDER BY chapter, verse
            "#,
            book_id
        )
        .fetch_all(&self.pool)
        .await
    }

    /// Get chapter info (verse counts) for a book
    pub async fn get_chapters_info(&self, book_id: i16) -> Result<Vec<ChapterInfo>, sqlx::Error> {
        sqlx::query_as!(
            ChapterInfo,
            r#"
            SELECT chapter, verse_count
            FROM bible_chapter_info
            WHERE book_id = $1
            ORDER BY chapter
            "#,
            book_id
        )
        .fetch_all(&self.pool)
        .await
    }
}

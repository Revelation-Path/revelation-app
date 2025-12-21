use shared::{SearchResult, Verse};
use sqlx::PgPool;

pub struct BibleSearch {
    pool: PgPool
}

impl BibleSearch {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    }

    /// Full-text search using PostgreSQL FTS
    pub async fn search(&self, query: &str, limit: i64) -> Result<Vec<SearchResult>, sqlx::Error> {
        let results = sqlx::query!(
            r#"
            SELECT
                v.id,
                v.book_id,
                v.chapter,
                v.verse,
                v.text,
                b.name_ru as book_name,
                ts_headline('russian', v.text, plainto_tsquery('russian', $1)) as headline
            FROM bible_verses v
            JOIN bible_books b ON b.id = v.book_id
            WHERE v.text_search @@ plainto_tsquery('russian', $1)
            ORDER BY ts_rank(v.text_search, plainto_tsquery('russian', $1)) DESC
            LIMIT $2
            "#,
            query,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results
            .into_iter()
            .map(|r| SearchResult {
                verse:      Verse {
                    id:      r.id,
                    book_id: r.book_id,
                    chapter: r.chapter,
                    verse:   r.verse,
                    text:    r.text
                },
                book_name:  r.book_name,
                highlights: Vec::new() // TODO: parse from headline
            })
            .collect())
    }

    /// Symphony - find all verses containing exact word
    pub async fn symphony(
        &self,
        word: &str,
        limit: i64
    ) -> Result<Vec<SearchResult>, sqlx::Error> {
        let results = sqlx::query!(
            r#"
            SELECT
                v.id,
                v.book_id,
                v.chapter,
                v.verse,
                v.text,
                b.name_ru as book_name
            FROM bible_word_index w
            JOIN bible_verses v ON v.id = w.verse_id
            JOIN bible_books b ON b.id = v.book_id
            WHERE w.word = lower($1)
            ORDER BY v.book_id, v.chapter, v.verse
            LIMIT $2
            "#,
            word,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results
            .into_iter()
            .map(|r| SearchResult {
                verse:      Verse {
                    id:      r.id,
                    book_id: r.book_id,
                    chapter: r.chapter,
                    verse:   r.verse,
                    text:    r.text
                },
                book_name:  r.book_name,
                highlights: Vec::new()
            })
            .collect())
    }

    /// Get word frequency for symphony
    pub async fn word_count(&self, word: &str) -> Result<i64, sqlx::Error> {
        let result = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as "count!"
            FROM bible_word_index
            WHERE word = lower($1)
            "#,
            word
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }
}

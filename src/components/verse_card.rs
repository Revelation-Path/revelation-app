//! Verse display components.

use leptos::prelude::*;
use revelation_bible::Verse;

/// Verse card for search results.
#[must_use]
#[component]
pub fn VerseCard(verse: Verse, book_name: String) -> impl IntoView {
    let text = verse.text;
    let chapter = verse.chapter;
    let verse_num = verse.verse;
    view! {
        <div class="card p-4">
            <p class="verse-text">{text}</p>
            <p class="text-sm mt-2" style="color: var(--color-text-muted)">
                {book_name} " " {chapter} ":" {verse_num}
            </p>
        </div>
    }
}

/// Verse list for reading chapters.
#[must_use]
#[component]
pub fn VerseList(verses: Vec<Verse>) -> impl IntoView {
    view! {
        <div class="space-y-1">
            {verses.into_iter().map(|v| view! {
                <div class="verse-container">
                    <span class="verse-number">{v.verse}</span>
                    <span class="verse-text">{v.text}</span>
                </div>
            }).collect::<Vec<_>>()}
        </div>
    }
}

//! Verse display components

use leptos::prelude::*;
use revelation_shared::Verse;

/// Verse card for search results
#[component]
pub fn VerseCard(verse: Verse, book_name: String) -> impl IntoView {
    view! {
        <div class="card p-4">
            <p class="verse-text">{verse.text.clone()}</p>
            <p class="text-sm mt-2" style="color: var(--color-text-muted)">
                {book_name} " " {verse.chapter}":"  {verse.verse}
            </p>
        </div>
    }
}

/// Verse list for reading chapters
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

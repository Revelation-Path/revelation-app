//! Feed page - community posts

use leptos::prelude::*;

use crate::components::{BottomNav, Header};

/// Feed page
#[component]
pub fn Feed() -> impl IntoView {
    view! {
        <div class="min-h-screen pb-20">
            <Header title="Лента"/>

            <main class="p-4 max-w-lg mx-auto">
                <div class="empty-state">
                    <div class="empty-state-icon">
                        <FeedIcon/>
                    </div>
                    <h2 class="empty-state-title">"Лента пуста"</h2>
                    <p class="empty-state-desc">
                        "Присоединитесь к церкви, чтобы видеть посты и общаться с братьями и сёстрами"
                    </p>
                    <a href="/churches" class="btn-primary mt-6 inline-block">
                        "Найти церковь"
                    </a>
                </div>
            </main>

            <BottomNav/>
        </div>
    }
}

#[component]
fn FeedIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="1.5" stroke-linecap="round"
             stroke-linejoin="round" width="32" height="32">
            <path d="M4 11a9 9 0 0 1 9 9"/>
            <path d="M4 4a16 16 0 0 1 16 16"/>
            <circle cx="5" cy="19" r="1"/>
        </svg>
    }
}

//! Loading components

use leptos::prelude::*;

/// Loading spinner
#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div class="flex items-center justify-center p-8">
            <div class="w-8 h-8 border-3 rounded-full animate-spin"
                 style="border-color: var(--color-parchment-300); border-top-color: var(--color-gold-500);">
            </div>
        </div>
    }
}

/// Full page loading
#[component]
pub fn LoadingPage() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center min-h-screen gap-4">
            <div class="w-12 h-12 border-3 rounded-full animate-spin"
                 style="border-color: var(--color-parchment-300); border-top-color: var(--color-gold-500);">
            </div>
            <p style="color: var(--color-text-muted)">"Загрузка..."</p>
        </div>
    }
}

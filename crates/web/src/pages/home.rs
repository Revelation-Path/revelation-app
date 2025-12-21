//! Home/Landing page

use leptos::prelude::*;
use leptos_router::components::A;

use crate::state::AppState;

/// Landing page component
#[component]
pub fn Home() -> impl IntoView {
    let state = expect_context::<AppState>();
    let has_profile = move || state.user.get().map(|u| u.name.is_some()).unwrap_or(false);

    view! {
        <div class="min-h-screen flex flex-col">
            // Header with logo
            <header class="safe-top px-6 py-4 flex items-center justify-between">
                <div class="flex items-center gap-2">
                    <CrossIcon/>
                    <span class="font-semibold text-lg">"Revelation"</span>
                </div>
            </header>

            // Main content
            <main class="flex-1 flex flex-col items-center justify-center px-6 pb-12">
                // Hero icon
                <div class="w-24 h-24 mb-8 rounded-3xl flex items-center justify-center shadow-glow-gold"
                     style="background: linear-gradient(135deg, var(--color-gold-500), var(--color-gold-600))">
                    <CrossIcon class="w-12 h-12 text-white"/>
                </div>

                // Title
                <h1 class="text-3xl font-bold mb-3 text-center text-gradient-gold">
                    "Слово Божие"
                </h1>

                // Subtitle
                <p class="text-center mb-10 max-w-sm" style="color: var(--color-text-secondary)">
                    "Читайте Библию и общайтесь с братьями и сёстрами во Христе"
                </p>

                // Features grid
                <div class="grid grid-cols-2 gap-3 mb-10 w-full max-w-sm">
                    <FeatureCard
                        icon=FeatureIcon::Book
                        title="Библия"
                        desc="Синодальный перевод"
                    />
                    <FeatureCard
                        icon=FeatureIcon::Search
                        title="Симфония"
                        desc="Поиск по словам"
                    />
                    <FeatureCard
                        icon=FeatureIcon::Church
                        title="Церкви"
                        desc="Найти общину"
                    />
                    <FeatureCard
                        icon=FeatureIcon::Heart
                        title="Общение"
                        desc="С верующими"
                    />
                </div>

                // Action buttons
                <div class="w-full max-w-sm space-y-3">
                    <Show
                        when=has_profile
                        fallback=|| view! {
                            <A href="/bible" attr:class="block w-full">
                                <button class="btn-primary w-full">
                                    "Читать Библию"
                                </button>
                            </A>
                            <A href="/onboarding" attr:class="block w-full">
                                <button class="btn-secondary w-full">
                                    "Создать профиль"
                                </button>
                            </A>
                        }
                    >
                        <A href="/bible" attr:class="block w-full">
                            <button class="btn-primary w-full">
                                "Читать Библию"
                            </button>
                        </A>
                        <A href="/feed" attr:class="block w-full">
                            <button class="btn-secondary w-full">
                                "Открыть ленту"
                            </button>
                        </A>
                    </Show>
                </div>
            </main>

            // Footer
            <footer class="py-6 text-center text-sm" style="color: var(--color-text-muted)">
                <p>"С любовью для церкви Христовой"</p>
            </footer>
        </div>
    }
}

#[derive(Clone, Copy)]
enum FeatureIcon {
    Book,
    Search,
    Church,
    Heart
}

#[component]
fn FeatureCard(icon: FeatureIcon, title: &'static str, desc: &'static str) -> impl IntoView {
    view! {
        <div class="feature-card">
            <div class="feature-card-icon">
                {match icon {
                    FeatureIcon::Book => view! { <BookIcon/> }.into_any(),
                    FeatureIcon::Search => view! { <SearchIcon/> }.into_any(),
                    FeatureIcon::Church => view! { <ChurchIcon/> }.into_any(),
                    FeatureIcon::Heart => view! { <HeartIcon/> }.into_any(),
                }}
            </div>
            <h3 class="feature-card-title">{title}</h3>
            <p class="feature-card-desc">{desc}</p>
        </div>
    }
}

// SVG Icons - inline for simplicity and performance
#[component]
fn CrossIcon(#[prop(optional)] class: &'static str) -> impl IntoView {
    view! {
        <svg class=class xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" width="24" height="24">
            <path d="M11 2v7H4v4h7v9h2v-9h7V9h-7V2z"/>
        </svg>
    }
}

#[component]
fn BookIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" width="24" height="24">
            <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/>
            <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/>
        </svg>
    }
}

#[component]
fn SearchIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" width="24" height="24">
            <circle cx="11" cy="11" r="8"/>
            <line x1="21" y1="21" x2="16.65" y2="16.65"/>
        </svg>
    }
}

#[component]
fn ChurchIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" width="24" height="24">
            <path d="M18 21V9l-6-4-6 4v12"/>
            <path d="M12 1v4"/>
            <path d="M9 5h6"/>
            <path d="M9 21v-6h6v6"/>
        </svg>
    }
}

#[component]
fn HeartIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" width="24" height="24">
            <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"/>
        </svg>
    }
}

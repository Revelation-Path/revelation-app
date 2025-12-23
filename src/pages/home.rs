//! Home/Landing page

use leptos::prelude::*;
use leptos_router::components::A;

use crate::state::AppState;

#[allow(dead_code)]
mod styles {
    stylance::import_crate_style!(pub common, "src/styles/common.module.css");
}
use styles::common;

/// Landing page component
#[component]
pub fn Home() -> impl IntoView {
    let state = expect_context::<AppState>();
    let has_profile = move || state.user.get().map(|u| u.name.is_some()).unwrap_or(false);

    view! {
        <div class=common::page style="display: flex; flex-direction: column;">
            // Header with logo
            <header style="padding: var(--space-md); padding-top: env(safe-area-inset-top); display: flex; align-items: center; justify-content: space-between;">
                <div style="display: flex; align-items: center; gap: var(--space-xs);">
                    <CrossIcon/>
                    <span class=common::fontSemibold style="font-size: var(--text-lg);">"Revelation"</span>
                </div>
            </header>

            // Main content
            <main style="flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; padding: var(--space-md); padding-bottom: var(--space-xl);">
                // Hero icon
                <div class=common::heroIcon>
                    <CrossIconLarge/>
                </div>

                // Title
                <h1 class=common::heroTitle>
                    "Слово Божие"
                </h1>

                // Subtitle
                <p class=common::heroSubtitle>
                    "Читайте Библию и общайтесь с братьями и сёстрами во Христе"
                </p>

                // Features grid
                <div class=common::grid2 style="max-width: 20rem; width: 100%; margin-bottom: var(--space-xl);">
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
                <div style="width: 100%; max-width: 20rem; display: flex; flex-direction: column; gap: var(--space-sm);">
                    <Show
                        when=has_profile
                        fallback=|| view! {
                            <A href="/bible" attr:class=common::btnPrimary attr:style="width: 100%; text-align: center;">
                                "Читать Библию"
                            </A>
                            <A href="/onboarding" attr:class=common::btnSecondary attr:style="width: 100%; text-align: center;">
                                "Создать профиль"
                            </A>
                        }
                    >
                        <A href="/bible" attr:class=common::btnPrimary attr:style="width: 100%; text-align: center;">
                            "Читать Библию"
                        </A>
                        <A href="/feed" attr:class=common::btnSecondary attr:style="width: 100%; text-align: center;">
                            "Открыть ленту"
                        </A>
                    </Show>
                </div>
            </main>

            // Footer
            <footer style="padding: var(--space-lg); text-align: center;">
                <p class=common::textMuted style="font-size: var(--text-sm);">
                    "С любовью для церкви Христовой"
                </p>
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
        <div class=common::featureCard>
            <div class=common::featureIcon>
                {match icon {
                    FeatureIcon::Book => view! { <BookIcon/> }.into_any(),
                    FeatureIcon::Search => view! { <SearchIcon/> }.into_any(),
                    FeatureIcon::Church => view! { <ChurchIcon/> }.into_any(),
                    FeatureIcon::Heart => view! { <HeartIcon/> }.into_any(),
                }}
            </div>
            <h3 class=common::featureTitle>{title}</h3>
            <p class=common::featureDesc>{desc}</p>
        </div>
    }
}

// SVG Icons
#[component]
fn CrossIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" width="24" height="24" style="color: var(--accent);">
            <path d="M11 2v7H4v4h7v9h2v-9h7V9h-7V2z"/>
        </svg>
    }
}

#[component]
fn CrossIconLarge() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" width="48" height="48">
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

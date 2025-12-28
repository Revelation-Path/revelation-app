//! Feed page - community posts

use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::{BottomNav, Header};

#[allow(dead_code)]
mod styles {
    stylance::import_crate_style!(pub common, "src/styles/common.module.css");
}
use styles::common;

/// Feed page
#[must_use]
#[component]
pub fn Feed() -> impl IntoView {
    view! {
        <div class=common::page>
            <Header title="Лента"/>

            <div class=common::container>
                <div class=common::emptyState>
                    <div class=common::emptyIcon>
                        <FeedIcon/>
                    </div>
                    <h2 class=common::emptyTitle>"Лента пуста"</h2>
                    <p class=common::emptyDesc>
                        "Присоединитесь к церкви, чтобы видеть посты и общаться с братьями и сёстрами"
                    </p>
                    <A href="/churches" attr:class=common::btnPrimary attr:style="margin-top: var(--space-lg);">
                        "Найти церковь"
                    </A>
                </div>
            </div>

            <BottomNav/>
        </div>
    }
}

#[must_use]
#[component]
fn FeedIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="1.5" stroke-linecap="round"
             stroke-linejoin="round" width="48" height="48">
            <path d="M4 11a9 9 0 0 1 9 9"/>
            <path d="M4 4a16 16 0 0 1 16 16"/>
            <circle cx="5" cy="19" r="1"/>
        </svg>
    }
}

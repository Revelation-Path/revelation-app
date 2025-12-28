//! 404 Not Found page

use leptos::prelude::*;
use leptos_router::components::A;

#[allow(dead_code)]
mod styles {
    stylance::import_crate_style!(pub common, "src/styles/common.module.css");
}
use styles::common;

#[must_use]
#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class=common::page style="display: flex; flex-direction: column; align-items: center; justify-content: center;">
            <div class=common::emptyState>
                <h1 class=common::text2xl style="font-weight: var(--font-bold); margin-bottom: var(--space-sm);">
                    "404"
                </h1>
                <p class=common::emptyDesc style="margin-bottom: var(--space-lg);">
                    "Страница не найдена"
                </p>
                <A href="/" attr:class=common::link>
                    "На главную"
                </A>
            </div>
        </div>
    }
}

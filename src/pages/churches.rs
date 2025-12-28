//! Churches pages

use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::{BottomNav, Header};

#[allow(dead_code)]
mod styles {
    stylance::import_crate_style!(pub common, "src/styles/common.module.css");
}
use styles::common;

#[must_use]
#[component]
pub fn Churches() -> impl IntoView {
    view! {
        <div class=common::page>
            <Header title="Церкви"/>

            <div class=common::container>
                <div class=common::emptyState>
                    <div class=common::emptyIcon>
                        <ChurchIcon/>
                    </div>
                    <h2 class=common::emptyTitle>"Пока нет церквей"</h2>
                    <p class=common::emptyDesc>
                        "Добавьте свою церковь или найдите общину рядом с вами"
                    </p>
                </div>

                <A href="/churches/create" attr:class=common::btnPrimary attr:style="width: 100%; text-align: center;">
                    "Добавить церковь"
                </A>
            </div>

            <BottomNav/>
        </div>
    }
}

#[must_use]
#[component]
pub fn ChurchDetail() -> impl IntoView {
    view! {
        <div class=common::page>
            <Header title="Церковь" back=true/>

            <div class=common::container>
                <div class=common::spinner>
                    <div class=common::spinnerCircle></div>
                </div>
            </div>

            <BottomNav/>
        </div>
    }
}

#[must_use]
#[component]
fn ChurchIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="1.5" stroke-linecap="round"
             stroke-linejoin="round" width="48" height="48">
            <path d="M18 21V9l-6-4-6 4v12"/>
            <path d="M12 1v4"/>
            <path d="M9 5h6"/>
            <path d="M9 21v-6h6v6"/>
        </svg>
    }
}

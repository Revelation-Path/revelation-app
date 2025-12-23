//! Daily reading page

use leptos::prelude::*;

use crate::{
    api,
    components::{BottomNav, Header, Loading, VerseList}
};

#[allow(dead_code)]
mod styles {
    stylance::import_crate_style!(pub common, "src/styles/common.module.css");
}
use styles::common;

#[component]
pub fn DailyReading() -> impl IntoView {
    let reading = LocalResource::new(|| async { api::get_today_reading().await.ok().flatten() });
    let response = RwSignal::new(String::new());

    view! {
        <div class=common::page>
            <Header title="Чтение на сегодня"/>

            <div class=common::container>
                <Suspense fallback=|| view! { <Loading/> }>
                    {move || reading.get().flatten().map(|reading| view! {
                        <div class=common::card>
                            <p class=common::textMuted style="margin-bottom: var(--space-sm);">
                                "День " {reading.day_of_year}
                            </p>
                            <VerseList verses=reading.verses/>
                        </div>

                        <div>
                            <label class=common::label>
                                "Что Господь говорит вам через этот отрывок?"
                            </label>
                            <textarea
                                placeholder="Поделитесь своими мыслями..."
                                class=common::textarea
                                prop:value=response
                                on:input=move |ev| response.set(event_target_value(&ev))
                            />
                        </div>

                        <button
                            class=common::btnPrimary
                            style="width: 100%;"
                            disabled=move || response.get().trim().is_empty()
                        >
                            "Отправить"
                        </button>
                    })}
                </Suspense>
            </div>

            <BottomNav/>
        </div>
    }
}

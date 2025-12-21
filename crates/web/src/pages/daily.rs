use leptos::prelude::*;

use crate::{
    api,
    components::{BottomNav, Header, Loading, VerseList}
};

#[component]
pub fn DailyReading() -> impl IntoView {
    let reading = LocalResource::new(|| async { api::get_today_reading().await.ok().flatten() });
    let response = RwSignal::new(String::new());

    view! {
        <div class="pb-20">
            <Header title="Чтение на сегодня"/>

            <div class="p-4 max-w-lg mx-auto">
                <Suspense fallback=|| view! { <Loading/> }>
                    {move || reading.get().flatten().map(|reading| view! {
                        <div class="bg-gray-800 rounded-lg p-4 mb-4">
                            <p class="text-sm text-gray-500 mb-2">
                                "День " {reading.day_of_year}
                            </p>
                            <VerseList verses=reading.verses/>
                        </div>

                        <div class="mb-4">
                            <label class="block text-sm text-gray-400 mb-2">
                                "Что Господь говорит вам через этот отрывок?"
                            </label>
                            <textarea
                                placeholder="Поделитесь своими мыслями..."
                                class="w-full bg-gray-800 border border-gray-700 rounded-lg px-4 py-3 min-h-32 focus:border-blue-600 focus:outline-none resize-none"
                                prop:value=response
                                on:input=move |ev| response.set(event_target_value(&ev))
                            />
                        </div>

                        <button
                            class="w-full bg-blue-600 hover:bg-blue-700 disabled:bg-gray-700 disabled:cursor-not-allowed text-white py-3 rounded-lg font-medium transition-colors"
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

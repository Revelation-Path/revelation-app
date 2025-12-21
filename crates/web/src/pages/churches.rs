use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::{BottomNav, Header};

#[component]
pub fn Churches() -> impl IntoView {
    view! {
        <div class="pb-20">
            <Header title="Церкви"/>

            <div class="p-4 max-w-lg mx-auto">
                <A
                    href="/churches/create"
                    attr:class="block w-full bg-blue-600 hover:bg-blue-700 text-white py-3 rounded-lg font-medium transition-colors text-center mb-4"
                >
                    "Добавить церковь"
                </A>

                <p class="text-gray-400 text-center py-8">
                    "Пока нет церквей в вашем городе. Будьте первым!"
                </p>
            </div>

            <BottomNav/>
        </div>
    }
}

#[component]
pub fn ChurchDetail() -> impl IntoView {
    view! {
        <div class="pb-20">
            <Header title="Церковь" back=true/>

            <div class="p-4 max-w-lg mx-auto">
                <p class="text-gray-400">"Загрузка..."</p>
            </div>

            <BottomNav/>
        </div>
    }
}

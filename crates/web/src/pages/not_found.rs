use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col items-center justify-center p-8">
            <h1 class="text-6xl font-bold mb-4">"404"</h1>
            <p class="text-gray-400 mb-8">"Страница не найдена"</p>
            <A
                href="/"
                attr:class="text-blue-500 hover:underline"
            >
                "На главную"
            </A>
        </div>
    }
}

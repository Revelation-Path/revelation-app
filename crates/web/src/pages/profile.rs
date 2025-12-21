use leptos::prelude::*;
use leptos_router::components::A;

use crate::{
    components::{BottomNav, Header},
    state::AppState
};

#[component]
pub fn Profile() -> impl IntoView {
    let state = expect_context::<AppState>();
    let user = state.user;

    view! {
        <div class="pb-20">
            <Header title="Профиль"/>

            <div class="p-4 max-w-lg mx-auto">
                <div class="bg-gray-800 rounded-lg p-4 mb-4">
                    <div class="flex items-center gap-4">
                        <div class="w-16 h-16 bg-gray-700 rounded-full flex items-center justify-center text-2xl">
                            "👤"
                        </div>
                        <div>
                            <h2 class="text-xl font-semibold">
                                {move || user.get().and_then(|u| u.name).unwrap_or_else(|| "Гость".to_string())}
                            </h2>
                            <p class="text-gray-400 text-sm">
                                "ID: " {move || state.user_id().to_string().chars().take(8).collect::<String>()} "..."
                            </p>
                        </div>
                    </div>
                </div>

                <div class="space-y-2">
                    <ProfileLink href="/settings" label="Настройки"/>
                    <ProfileLink href="/profile/churches" label="Мои церкви"/>
                    <ProfileLink href="/profile/payments" label="Пожертвования"/>
                </div>

                <div class="mt-8">
                    <h3 class="text-sm text-gray-500 uppercase mb-2">"Привязать аккаунт"</h3>
                    <div class="space-y-2">
                        <BindButton label="Telegram" icon="📱" bound=false/>
                        <BindButton label="Email" icon="📧" bound=false/>
                        <BindButton label="Телефон" icon="📞" bound=false/>
                    </div>
                </div>
            </div>

            <BottomNav/>
        </div>
    }
}

#[component]
fn ProfileLink(href: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <A
            href=href
            attr:class="block bg-gray-800 rounded-lg px-4 py-3 hover:bg-gray-700 transition-colors"
        >
            {label}
        </A>
    }
}

#[component]
fn BindButton(label: &'static str, icon: &'static str, bound: bool) -> impl IntoView {
    view! {
        <button
            class="w-full bg-gray-800 rounded-lg px-4 py-3 hover:bg-gray-700 transition-colors text-left flex items-center justify-between"
            disabled=bound
        >
            <span>
                <span class="mr-2">{icon}</span>
                {label}
            </span>
            {bound.then(|| view! { <span class="text-green-500">"✓"</span> })}
        </button>
    }
}

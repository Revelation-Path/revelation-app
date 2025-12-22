use leptos::prelude::*;

use crate::components::Header;

#[component]
pub fn Settings() -> impl IntoView {
    view! {
        <div>
            <Header title="Настройки" back=true/>

            <div class="p-4 max-w-lg mx-auto">
                <div class="space-y-4">
                    <SettingToggle
                        label="Уведомления"
                        description="Получать ежедневные стихи"
                        enabled=true
                    />
                    <SettingToggle
                        label="Тёмная тема"
                        description="Всегда использовать тёмную тему"
                        enabled=true
                    />
                </div>

                // Version info
                <div class="mt-8 pt-4 border-t border-[var(--color-border)]">
                    <div class="text-center text-sm text-[var(--color-text-muted)]">
                        <p class="font-medium">"Revelation"</p>
                        <p>"Версия " {env!("CARGO_PKG_VERSION")}</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn SettingToggle(label: &'static str, description: &'static str, enabled: bool) -> impl IntoView {
    let is_enabled = RwSignal::new(enabled);

    view! {
        <div class="flex items-center justify-between bg-gray-800 rounded-lg p-4">
            <div>
                <p class="font-medium">{label}</p>
                <p class="text-sm text-gray-400">{description}</p>
            </div>
            <button
                class=move || format!(
                    "w-12 h-6 rounded-full transition-colors {}",
                    if is_enabled.get() { "bg-blue-600" } else { "bg-gray-600" }
                )
                on:click=move |_| is_enabled.update(|v| *v = !*v)
            >
                <div
                    class=move || format!(
                        "w-5 h-5 bg-white rounded-full transition-transform {}",
                        if is_enabled.get() { "translate-x-6" } else { "translate-x-0.5" }
                    )
                />
            </button>
        </div>
    }
}

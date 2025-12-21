//! Onboarding flow

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use shared::Gender;

/// Onboarding page with steps
#[component]
pub fn Onboarding() -> impl IntoView {
    let step = RwSignal::new(1);
    let name = RwSignal::new(String::new());
    let gender = RwSignal::new(Option::<Gender>::None);

    view! {
        <div class="min-h-screen flex flex-col">
            // Header with progress
            <header class="header safe-top px-6 py-4">
                <div class="max-w-lg mx-auto">
                    // Progress steps
                    <div class="flex gap-2">
                        {(1..=3).map(|i| view! {
                            <div class=move || format!(
                                "progress-step {}",
                                if step.get() >= i { "completed" } else { "pending" }
                            )/>
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            </header>

            // Content
            <main class="flex-1 flex flex-col p-6 max-w-lg mx-auto w-full">
                <Show when=move || step.get() == 1>
                    <Step1 name=name on_next=move || step.set(2)/>
                </Show>

                <Show when=move || step.get() == 2>
                    <Step2
                        gender=gender
                        on_next=move || step.set(3)
                        on_back=move || step.set(1)
                    />
                </Show>

                <Show when=move || step.get() == 3>
                    <Step3 on_back=move || step.set(2)/>
                </Show>
            </main>
        </div>
    }
}

#[component]
fn Step1(name: RwSignal<String>, on_next: impl Fn() + 'static) -> impl IntoView {
    view! {
        <div class="flex-1 flex flex-col">
            // Content
            <div class="flex-1">
                <div class="mb-2">
                    <span class="badge badge-gold">"Шаг 1 из 3"</span>
                </div>
                <h1 class="text-2xl font-bold mb-2">"Как вас зовут?"</h1>
                <p class="mb-6" style="color: var(--color-text-muted)">
                    "Это имя будут видеть другие верующие"
                </p>

                <input
                    type="text"
                    placeholder="Ваше имя"
                    class="input"
                    prop:value=name
                    on:input=move |ev| name.set(event_target_value(&ev))
                />
            </div>

            // Action
            <div class="pt-6">
                <button
                    class="btn-primary w-full"
                    disabled=move || name.get().trim().is_empty()
                    on:click=move |_| on_next()
                >
                    "Далее"
                </button>
            </div>
        </div>
    }
}

#[component]
fn Step2(
    gender: RwSignal<Option<Gender>>,
    on_next: impl Fn() + 'static,
    on_back: impl Fn() + 'static
) -> impl IntoView {
    view! {
        <div class="flex-1 flex flex-col">
            // Content
            <div class="flex-1">
                <div class="mb-2">
                    <span class="badge badge-gold">"Шаг 2 из 3"</span>
                </div>
                <h1 class="text-2xl font-bold mb-2">"Ваш пол"</h1>
                <p class="mb-6" style="color: var(--color-text-muted)">
                    "Для правильного обращения"
                </p>

                <div class="space-y-3">
                    <GenderOption
                        label="Брат"
                        sublabel="Мужской"
                        value=Gender::Male
                        selected=gender
                    />
                    <GenderOption
                        label="Сестра"
                        sublabel="Женский"
                        value=Gender::Female
                        selected=gender
                    />
                </div>
            </div>

            // Actions
            <div class="pt-6 flex gap-3">
                <button
                    class="btn-secondary flex-1"
                    on:click=move |_| on_back()
                >
                    "Назад"
                </button>
                <button
                    class="btn-primary flex-1"
                    disabled=move || gender.get().is_none()
                    on:click=move |_| on_next()
                >
                    "Далее"
                </button>
            </div>
        </div>
    }
}

#[component]
fn GenderOption(
    label: &'static str,
    sublabel: &'static str,
    value: Gender,
    selected: RwSignal<Option<Gender>>
) -> impl IntoView {
    let is_selected = move || selected.get() == Some(value);

    view! {
        <button
            class=move || format!(
                "selection-card w-full text-left {}",
                if is_selected() { "selected" } else { "" }
            )
            on:click=move |_| selected.set(Some(value))
        >
            <div class="font-semibold">{label}</div>
            <div class="text-sm" style="color: var(--color-text-muted)">{sublabel}</div>
        </button>
    }
}

#[component]
fn Step3(on_back: impl Fn() + 'static) -> impl IntoView {
    let navigate = use_navigate();

    view! {
        <div class="flex-1 flex flex-col">
            // Content
            <div class="flex-1 flex flex-col items-center justify-center text-center">
                // Success icon
                <div class="w-20 h-20 mb-6 rounded-full flex items-center justify-center"
                     style="background: var(--color-gold-100)">
                    <CheckIcon/>
                </div>

                <h1 class="text-2xl font-bold mb-2">"Добро пожаловать!"</h1>
                <p class="mb-6" style="color: var(--color-text-muted)">
                    "Теперь вы можете изучать Слово Божие и общаться с братьями и сёстрами"
                </p>
            </div>

            // Actions
            <div class="pt-6 space-y-3">
                <button
                    class="btn-primary w-full"
                    on:click=move |_| navigate("/bible", Default::default())
                >
                    "Открыть Библию"
                </button>
                <button
                    class="btn-secondary w-full"
                    on:click=move |_| on_back()
                >
                    "Назад"
                </button>
            </div>
        </div>
    }
}

#[component]
fn CheckIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="var(--color-gold-600)" stroke-width="2.5" stroke-linecap="round"
             stroke-linejoin="round" width="40" height="40">
            <polyline points="20 6 9 17 4 12"/>
        </svg>
    }
}

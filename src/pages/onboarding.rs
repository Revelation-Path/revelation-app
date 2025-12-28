//! Onboarding flow

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use revelation_user::Gender;

#[allow(dead_code)]
mod styles {
    stylance::import_crate_style!(pub common, "src/styles/common.module.css");
}
use styles::common;

/// Onboarding page with steps
#[must_use]
#[component]
pub fn Onboarding() -> impl IntoView {
    let step = RwSignal::new(1);
    let name = RwSignal::new(String::new());
    let gender = RwSignal::new(Option::<Gender>::None);

    view! {
        <div class=common::page style="display: flex; flex-direction: column;">
            // Header with progress
            <header style="padding: var(--space-md); padding-top: env(safe-area-inset-top);">
                <div style="max-width: 32rem; margin: 0 auto;">
                    <div class=common::progressSteps>
                        {(1..=3).map(|i| view! {
                            <div class={move || if step.get() >= i {
                                format!("{} {}", common::progressStep, common::progressStepComplete)
                            } else {
                                common::progressStep.to_string()
                            }}/>
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            </header>

            // Content
            <main style="flex: 1; display: flex; flex-direction: column; padding: var(--space-md); max-width: 32rem; margin: 0 auto; width: 100%;">
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

#[must_use]
#[component]
fn Step1(name: RwSignal<String>, on_next: impl Fn() + 'static) -> impl IntoView {
    view! {
        <div style="flex: 1; display: flex; flex-direction: column;">
            // Content
            <div style="flex: 1;">
                <div style="margin-bottom: var(--space-xs);">
                    <span class=common::badge>"Шаг 1 из 3"</span>
                </div>
                <h1 class=common::heroTitle style="text-align: left; margin-bottom: var(--space-xs);">"Как вас зовут?"</h1>
                <p class=common::textMuted style="margin-bottom: var(--space-lg);">
                    "Это имя будут видеть другие верующие"
                </p>

                <input
                    type="text"
                    placeholder="Ваше имя"
                    class=common::input
                    prop:value=name
                    on:input=move |ev| name.set(event_target_value(&ev))
                />
            </div>

            // Action
            <div style="padding-top: var(--space-lg);">
                <button
                    class=common::btnPrimary
                    style="width: 100%;"
                    disabled=move || name.get().trim().is_empty()
                    on:click=move |_| on_next()
                >
                    "Далее"
                </button>
            </div>
        </div>
    }
}

#[must_use]
#[component]
fn Step2(
    gender: RwSignal<Option<Gender>>,
    on_next: impl Fn() + 'static,
    on_back: impl Fn() + 'static
) -> impl IntoView {
    view! {
        <div style="flex: 1; display: flex; flex-direction: column;">
            // Content
            <div style="flex: 1;">
                <div style="margin-bottom: var(--space-xs);">
                    <span class=common::badge>"Шаг 2 из 3"</span>
                </div>
                <h1 class=common::heroTitle style="text-align: left; margin-bottom: var(--space-xs);">"Ваш пол"</h1>
                <p class=common::textMuted style="margin-bottom: var(--space-lg);">
                    "Для правильного обращения"
                </p>

                <div style="display: flex; flex-direction: column; gap: var(--space-sm);">
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
            <div style="padding-top: var(--space-lg); display: flex; gap: var(--space-sm);">
                <button
                    class=common::btnSecondary
                    style="flex: 1;"
                    on:click=move |_| on_back()
                >
                    "Назад"
                </button>
                <button
                    class=common::btnPrimary
                    style="flex: 1;"
                    disabled=move || gender.get().is_none()
                    on:click=move |_| on_next()
                >
                    "Далее"
                </button>
            </div>
        </div>
    }
}

#[must_use]
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
            class=move || if is_selected() {
                format!("{} {}", common::selectionCard, common::selectionCardSelected)
            } else {
                common::selectionCard.to_string()
            }
            on:click=move |_| selected.set(Some(value))
        >
            <div class=common::fontSemibold>{label}</div>
            <div class=common::textMuted style="font-size: var(--text-sm);">{sublabel}</div>
        </button>
    }
}

#[must_use]
#[component]
fn Step3(on_back: impl Fn() + 'static) -> impl IntoView {
    let navigate = use_navigate();

    view! {
        <div style="flex: 1; display: flex; flex-direction: column;">
            // Content
            <div style="flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; text-align: center;">
                // Success icon
                <div class=common::heroIcon style="margin-bottom: var(--space-lg);">
                    <CheckIcon/>
                </div>

                <h1 class=common::heroTitle>"Добро пожаловать!"</h1>
                <p class=common::heroSubtitle>
                    "Теперь вы можете изучать Слово Божие и общаться с братьями и сёстрами"
                </p>
            </div>

            // Actions
            <div style="padding-top: var(--space-lg); display: flex; flex-direction: column; gap: var(--space-sm);">
                <button
                    class=common::btnPrimary
                    style="width: 100%;"
                    on:click=move |_| navigate("/bible", leptos_router::NavigateOptions::default())
                >
                    "Открыть Библию"
                </button>
                <button
                    class=common::btnSecondary
                    style="width: 100%;"
                    on:click=move |_| on_back()
                >
                    "Назад"
                </button>
            </div>
        </div>
    }
}

#[must_use]
#[component]
fn CheckIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2.5" stroke-linecap="round"
             stroke-linejoin="round" width="40" height="40">
            <polyline points="20 6 9 17 4 12"/>
        </svg>
    }
}

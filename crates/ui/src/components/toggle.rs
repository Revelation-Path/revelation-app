//! Toggle button component - pill-shaped filter toggle

use leptos::prelude::*;

use super::icon::{Icon, IconSize, IconType};

stylance::import_crate_style!(styles, "src/styles/toggle.module.css");

/// Toggle button - pill-shaped button that can be on/off
#[component]
pub fn Toggle(
    /// Current active state
    #[prop(into)]
    active: Signal<bool>,
    /// Label when inactive
    label_off: &'static str,
    /// Label when active
    label_on: &'static str,
    /// Optional icon
    #[prop(optional)]
    icon: Option<IconType>,
    /// Toggle callback
    #[prop(optional)]
    on_toggle: Option<Callback<bool>>
) -> impl IntoView {
    let handle_click = move |_| {
        if let Some(ref cb) = on_toggle {
            cb.run(!active.get());
        }
    };

    view! {
        <button
            class=move || {
                if active.get() {
                    format!("{} {}", styles::toggle, styles::toggleActive)
                } else {
                    styles::toggle.to_string()
                }
            }
            on:click=handle_click
        >
            {icon.map(|i| view! { <Icon icon=i size=IconSize::Small/> })}
            <span>{move || if active.get() { label_on } else { label_off }}</span>
        </button>
    }
}

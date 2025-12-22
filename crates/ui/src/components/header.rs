//! Universal page header component

use leptos::prelude::*;

use crate::Icon;

stylance::import_crate_style!(styles, "src/styles/header.module.css");

/// Page header with navigation and actions
#[component]
pub fn PageHeader(
    /// Primary title (book name, songbook name, etc.)
    #[prop(into)]
    title: Signal<String>,
    /// Secondary label (chapter number, song number, etc.)
    #[prop(optional, into)]
    subtitle: Option<Signal<String>>,
    /// Progress value 0.0-1.0 (for scroll progress)
    #[prop(optional, into)]
    progress: Option<Signal<f64>>,
    /// Progress bar color
    #[prop(optional)]
    progress_color: Option<&'static str>,
    /// On title click callback
    #[prop(optional, into)]
    on_title_click: Option<Callback<()>>,
    /// On subtitle click callback
    #[prop(optional, into)]
    on_subtitle_click: Option<Callback<()>>,
    /// Left slot for back button etc.
    #[prop(optional)]
    left: Option<Children>,
    /// Right slot for action buttons
    #[prop(optional)]
    right: Option<Children>
) -> impl IntoView {
    let progress_style = move || {
        if let Some(p) = &progress {
            let value = p.get();
            let color = progress_color.unwrap_or("var(--accent-soft)");
            format!(
                "width: {}%; opacity: {}; background: {};",
                value * 100.0,
                if value > 0.0 { "1" } else { "0" },
                color
            )
        } else {
            "opacity: 0;".to_string()
        }
    };

    view! {
        <header class=styles::header>
            <div class=styles::progress style=progress_style></div>

            <div class=styles::left>
                {left.map(|children| children())}
            </div>

            <div class=styles::title>
                {move || {
                    let title_text = title.get();
                    if let Some(ref on_click) = on_title_click {
                        let on_click = on_click.clone();
                        view! {
                            <button
                                class=styles::titleBtn
                                on:click=move |_| on_click.run(())
                            >
                                {title_text}
                                <Icon icon=crate::IconType::ChevronDown size=crate::IconSize::Small/>
                            </button>
                        }.into_any()
                    } else {
                        view! {
                            <span class=styles::titleText>{title_text}</span>
                        }.into_any()
                    }
                }}

                {move || {
                    subtitle.as_ref().map(|s| {
                        let subtitle_text = s.get();
                        if let Some(ref on_click) = on_subtitle_click {
                            let on_click = on_click.clone();
                            view! {
                                <button
                                    class=styles::subtitleBtn
                                    on:click=move |_| on_click.run(())
                                >
                                    {subtitle_text}
                                    <Icon icon=crate::IconType::ChevronDown size=crate::IconSize::Small/>
                                </button>
                            }.into_any()
                        } else {
                            view! {
                                <span class=styles::subtitleText>{subtitle_text}</span>
                            }.into_any()
                        }
                    })
                }}
            </div>

            <div class=styles::right>
                {right.map(|children| children())}
            </div>
        </header>
    }
}

/// Header icon button
#[component]
pub fn HeaderButton(
    /// Icon type
    icon: crate::IconType,
    /// On click handler
    #[prop(into)]
    on_click: Callback<()>,
    /// Additional class
    #[prop(optional)]
    class: &'static str
) -> impl IntoView {
    let classes = format!("{} {}", styles::btn, class);

    view! {
        <button class=classes on:click=move |_| on_click.run(())>
            <Icon icon=icon size=crate::IconSize::Medium/>
        </button>
    }
}

//! Button component with multiple variants

use leptos::prelude::*;

/// Visual style of the button
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonVariant {
    /// Primary action button with gradient
    #[default]
    Primary,
    /// Secondary button with subtle background
    Secondary,
    /// Ghost button with transparent background
    Ghost,
    /// Danger/destructive action
    Danger,
    /// Success action
    Success
}

impl ButtonVariant {
    fn class(&self) -> &'static str {
        match self {
            Self::Primary => "ui-btn--primary",
            Self::Secondary => "ui-btn--secondary",
            Self::Ghost => "ui-btn--ghost",
            Self::Danger => "ui-btn--danger",
            Self::Success => "ui-btn--success"
        }
    }
}

/// Button size
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large
}

impl ButtonSize {
    fn class(&self) -> &'static str {
        match self {
            Self::Small => "ui-btn--sm",
            Self::Medium => "ui-btn--md",
            Self::Large => "ui-btn--lg"
        }
    }
}

/// Universal button component
#[component]
pub fn Button(
    /// Button variant controlling colors
    #[prop(default = ButtonVariant::Primary)]
    variant: ButtonVariant,
    /// Button size
    #[prop(default = ButtonSize::Medium)]
    size: ButtonSize,
    /// Additional CSS classes
    #[prop(optional)]
    class: &'static str,
    /// Disabled state
    #[prop(default = false)]
    disabled: bool,
    /// Full width button
    #[prop(default = false)]
    full_width: bool,
    /// Loading state
    #[prop(default = false)]
    loading: bool,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Callback<()>>,
    /// Button contents
    children: Children
) -> impl IntoView {
    let handle_click = move |_| {
        if !disabled
            && !loading
            && let Some(ref cb) = on_click
        {
            cb.run(());
        }
    };

    let classes = {
        let mut classes = vec!["ui-btn", variant.class(), size.class()];
        if full_width {
            classes.push("ui-btn--full");
        }
        if loading {
            classes.push("ui-btn--loading");
        }
        if !class.is_empty() {
            classes.push(class);
        }
        classes.join(" ")
    };

    let content = children();

    view! {
        <button
            class=classes
            disabled=disabled || loading
            on:click=handle_click
        >
            {if loading {
                view! {
                    <span class="ui-btn__spinner"></span>
                    <span class="ui-btn__content ui-btn__content--loading">{content}</span>
                }.into_any()
            } else {
                view! {
                    <span class="ui-btn__content">{content}</span>
                }.into_any()
            }}
        </button>
    }
}

/// Icon button - circular button with icon
#[component]
pub fn IconButton(
    /// Button variant
    #[prop(default = ButtonVariant::Ghost)]
    variant: ButtonVariant,
    /// Button size
    #[prop(default = ButtonSize::Medium)]
    size: ButtonSize,
    /// Additional CSS classes
    #[prop(optional)]
    class: &'static str,
    /// Disabled state
    #[prop(default = false)]
    disabled: bool,
    /// Aria label for accessibility
    #[prop(optional)]
    label: &'static str,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Callback<()>>,
    /// Icon content
    children: Children
) -> impl IntoView {
    let handle_click = move |_| {
        if !disabled && let Some(ref cb) = on_click {
            cb.run(());
        }
    };

    let classes = {
        let mut classes = vec!["ui-btn", "ui-btn--icon", variant.class(), size.class()];
        if !class.is_empty() {
            classes.push(class);
        }
        classes.join(" ")
    };

    view! {
        <button
            class=classes
            disabled=disabled
            aria-label=label
            on:click=handle_click
        >
            {children()}
        </button>
    }
}

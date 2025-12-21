//! Card component with variants

use leptos::prelude::*;

/// Card variant
#[derive(Clone, Copy, PartialEq, Default)]
pub enum CardVariant {
    /// Default glass effect
    #[default]
    Default,
    /// Elevated with shadow
    Elevated,
    /// Outlined border only
    Outlined,
    /// Solid background
    Solid
}

impl CardVariant {
    fn class(&self) -> &'static str {
        match self {
            Self::Default => "ui-card--default",
            Self::Elevated => "ui-card--elevated",
            Self::Outlined => "ui-card--outlined",
            Self::Solid => "ui-card--solid"
        }
    }
}

/// Card padding size
#[derive(Clone, Copy, PartialEq, Default)]
pub enum CardPadding {
    None,
    Small,
    #[default]
    Medium,
    Large
}

impl CardPadding {
    fn class(&self) -> &'static str {
        match self {
            Self::None => "ui-card--p-none",
            Self::Small => "ui-card--p-sm",
            Self::Medium => "ui-card--p-md",
            Self::Large => "ui-card--p-lg"
        }
    }
}

/// Universal card component
#[component]
pub fn Card(
    /// Card variant
    #[prop(default = CardVariant::Default)]
    variant: CardVariant,
    /// Card padding
    #[prop(default = CardPadding::Medium)]
    padding: CardPadding,
    /// Additional CSS classes
    #[prop(optional)]
    class: &'static str,
    /// Hover effect
    #[prop(default = false)]
    hoverable: bool,
    /// Clickable card
    #[prop(default = false)]
    clickable: bool,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Callback<()>>,
    /// Card contents
    children: Children
) -> impl IntoView {
    let handle_click = move |_| {
        if clickable {
            if let Some(cb) = on_click {
                cb.run(());
            }
        }
    };

    let classes = move || {
        let mut classes = vec!["ui-card", variant.class(), padding.class()];
        if hoverable {
            classes.push("ui-card--hoverable");
        }
        if clickable {
            classes.push("ui-card--clickable");
        }
        if !class.is_empty() {
            classes.push(class);
        }
        classes.join(" ")
    };

    view! {
        <div
            class=classes
            role={if clickable { Some("button") } else { None }}
            tabindex={if clickable { Some(0) } else { None }}
            on:click=handle_click
        >
            {children()}
        </div>
    }
}

/// Card header section
#[component]
pub fn CardHeader(
    /// Additional CSS classes
    #[prop(optional)]
    class: &'static str,
    /// Header contents
    children: Children
) -> impl IntoView {
    let classes = format!("ui-card__header {}", class);
    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

/// Card body section
#[component]
pub fn CardBody(
    /// Additional CSS classes
    #[prop(optional)]
    class: &'static str,
    /// Body contents
    children: Children
) -> impl IntoView {
    let classes = format!("ui-card__body {}", class);
    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

/// Card footer section
#[component]
pub fn CardFooter(
    /// Additional CSS classes
    #[prop(optional)]
    class: &'static str,
    /// Footer contents
    children: Children
) -> impl IntoView {
    let classes = format!("ui-card__footer {}", class);
    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

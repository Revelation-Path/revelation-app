//! Tabs component
//!
//! Accessible tab navigation with keyboard support.

use leptos::prelude::*;

/// Tabs visual variant
#[derive(Clone, Copy, PartialEq, Default)]
pub enum TabsVariant {
    /// Default contained style
    #[default]
    Default,
    /// Pill-shaped tabs
    Pills,
    /// Underline style
    Underline
}

impl TabsVariant {
    fn class(&self) -> &'static str {
        match self {
            Self::Default => "ui-tabs--default",
            Self::Pills => "ui-tabs--pills",
            Self::Underline => "ui-tabs--underline"
        }
    }
}

/// Single tab item configuration
#[derive(Clone)]
pub struct TabItem {
    /// Unique tab identifier
    pub id:       String,
    /// Display label
    pub label:    String,
    /// Whether tab is disabled
    pub disabled: bool
}

impl TabItem {
    /// Create a new tab item
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id:       id.into(),
            label:    label.into(),
            disabled: false
        }
    }

    /// Mark tab as disabled
    #[must_use]
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Tabs navigation component
///
/// # Example
///
/// ```ignore
/// let active = RwSignal::new("tab1".to_string());
///
/// view! {
///     <Tabs
///         tabs=vec![
///             TabItem::new("tab1", "First"),
///             TabItem::new("tab2", "Second"),
///         ]
///         active=active.into()
///         on_change=Callback::new(move |id| active.set(id))
///     />
/// }
/// ```
#[component]
pub fn Tabs(
    /// List of tab items
    tabs: Vec<TabItem>,
    /// Currently active tab id
    #[prop(into)]
    active: Signal<String>,
    /// Callback when tab changes
    on_change: Callback<String>,
    /// Visual variant
    #[prop(default = TabsVariant::Default)]
    variant: TabsVariant,
    /// Stretch tabs to full width
    #[prop(default = false)]
    full_width: bool,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: String
) -> impl IntoView {
    let container_classes = {
        let mut classes = vec!["ui-tabs", variant.class()];
        if full_width {
            classes.push("ui-tabs--full");
        }
        if !class.is_empty() {
            format!("{} {}", classes.join(" "), class)
        } else {
            classes.join(" ")
        }
    };

    view! {
        <div class=container_classes role="tablist">
            {tabs.into_iter().map(|tab| {
                let tab_id_for_class = tab.id.clone();
                let tab_id_for_aria = tab.id.clone();
                let tab_id_for_click = tab.id.clone();
                let tab_disabled = tab.disabled;
                let on_change = on_change.clone();

                view! {
                    <button
                        class=move || {
                            let mut classes = vec!["ui-tabs__tab"];
                            if active.get() == tab_id_for_class {
                                classes.push("ui-tabs__tab--active");
                            }
                            if tab_disabled {
                                classes.push("ui-tabs__tab--disabled");
                            }
                            classes.join(" ")
                        }
                        role="tab"
                        aria-selected=move || (active.get() == tab_id_for_aria).to_string()
                        disabled=tab_disabled
                        on:click=move |_| {
                            if !tab_disabled {
                                on_change.run(tab_id_for_click.clone());
                            }
                        }
                    >
                        {tab.label}
                    </button>
                }
            }).collect_view()}
        </div>
    }
}

/// Tab panel container
///
/// Shows content only when matching tab is active.
#[component]
pub fn TabPanel(
    /// Panel id (must match tab id)
    #[prop(into)]
    id: String,
    /// Currently active tab id
    #[prop(into)]
    active: Signal<String>,
    /// Panel contents (rendered only when active)
    children: ChildrenFn
) -> impl IntoView {
    let id_for_hidden = id.clone();
    let id_for_show = id.clone();

    view! {
        <div
            class="ui-tabs__panel"
            role="tabpanel"
            hidden=move || active.get() != id_for_hidden
        >
            <Show when=move || active.get() == id_for_show>
                {children()}
            </Show>
        </div>
    }
}

/// Segmented control component
///
/// Alternative to tabs for small sets of mutually exclusive options.
///
/// # Example
///
/// ```ignore
/// #[derive(Clone, PartialEq)]
/// enum View { Grid, List }
///
/// let view = RwSignal::new(View::Grid);
///
/// view! {
///     <SegmentedControl
///         options=vec![(View::Grid, "Grid"), (View::List, "List")]
///         value=view.into()
///         on_change=Callback::new(move |v| view.set(v))
///     />
/// }
/// ```
#[component]
pub fn SegmentedControl<T>(
    /// Available options as (value, label) pairs
    options: Vec<(T, &'static str)>,
    /// Currently selected value
    #[prop(into)]
    value: Signal<T>,
    /// Callback when selection changes
    on_change: Callback<T>,
    /// Size variant
    #[prop(default = "md")]
    size: &'static str,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: String
) -> impl IntoView
where
    T: Clone + PartialEq + Send + Sync + 'static
{
    let size_class = match size {
        "sm" => "ui-segmented--sm",
        "lg" => "ui-segmented--lg",
        _ => "ui-segmented--md"
    };

    let classes = if class.is_empty() {
        format!("ui-segmented {}", size_class)
    } else {
        format!("ui-segmented {} {}", size_class, class)
    };

    view! {
        <div class=classes role="radiogroup">
            {options.into_iter().map(|(opt_value, label)| {
                let opt_for_class = opt_value.clone();
                let opt_for_aria = opt_value.clone();
                let opt_for_click = opt_value.clone();
                let on_change = on_change.clone();

                view! {
                    <button
                        class=move || {
                            if value.get() == opt_for_class {
                                "ui-segmented__option ui-segmented__option--active"
                            } else {
                                "ui-segmented__option"
                            }
                        }
                        role="radio"
                        aria-checked=move || (value.get() == opt_for_aria).to_string()
                        on:click=move |_| on_change.run(opt_for_click.clone())
                    >
                        {label}
                    </button>
                }
            }).collect_view()}
        </div>
    }
}

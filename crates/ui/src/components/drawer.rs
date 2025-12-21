//! Bottom drawer (sheet) component
//!
//! A mobile-first bottom sheet drawer component with smooth animations.

use leptos::prelude::*;

/// Drawer height variants
#[derive(Clone, Copy, PartialEq, Default)]
pub enum DrawerHeight {
    /// Auto height based on content (max 75vh)
    #[default]
    Auto,
    /// Fixed 50% of viewport
    Half,
    /// Fixed 75% of viewport
    ThreeQuarters,
    /// Full screen
    Full
}

impl DrawerHeight {
    fn style(&self) -> &'static str {
        match self {
            Self::Auto => "height: auto; max-height: 75vh;",
            Self::Half => "height: 50vh;",
            Self::ThreeQuarters => "height: 75vh;",
            Self::Full => "height: 100vh;"
        }
    }
}

/// Bottom drawer component
///
/// A sliding bottom sheet that overlays the page content.
/// Uses `ChildrenFn` because children may be rendered multiple times
/// when the drawer opens and closes.
///
/// # Example
///
/// ```ignore
/// let open = RwSignal::new(false);
///
/// view! {
///     <Button on_click=Callback::new(move |_| open.set(true))>
///         "Open Drawer"
///     </Button>
///     <Drawer
///         open=open.into()
///         on_close=Callback::new(move |_| open.set(false))
///     >
///         <DrawerHeader title="Settings" />
///         <DrawerBody>
///             <p>"Drawer content here"</p>
///         </DrawerBody>
///     </Drawer>
/// }
/// ```
#[component]
pub fn Drawer(
    /// Whether the drawer is open
    #[prop(into)]
    open: Signal<bool>,
    /// Callback when drawer requests to close
    on_close: Callback<()>,
    /// Drawer height
    #[prop(default = DrawerHeight::Auto)]
    height: DrawerHeight,
    /// Aria label for accessibility
    #[prop(optional, into)]
    label: Option<String>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: String,
    /// Show drag handle bar
    #[prop(default = true)]
    show_handle: bool,
    /// Drawer contents (can be rendered multiple times)
    children: ChildrenFn
) -> impl IntoView {
    let leaving = RwSignal::new(false);
    let aria_label = label.unwrap_or_else(|| "Dialog".to_string());

    let handle_overlay_click = move |_| {
        if !leaving.get() {
            leaving.set(true);
        }
    };

    let handle_animation_end = {
        move |_: web_sys::AnimationEvent| {
            if leaving.get() {
                leaving.set(false);
                on_close.run(());
            }
        }
    };

    let stop_propagation = |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
    };

    view! {
        <Show when=move || open.get()>
            {
                let overlay_classes = move || {
                    if leaving.get() {
                        "ui-drawer-overlay ui-drawer-overlay--leave"
                    } else {
                        "ui-drawer-overlay"
                    }
                };

                let drawer_classes = {
                    let base = if class.is_empty() {
                        "ui-drawer".to_string()
                    } else {
                        format!("ui-drawer {}", class)
                    };
                    move || {
                        if leaving.get() {
                            format!("{} ui-drawer--leave", base)
                        } else {
                            base.clone()
                        }
                    }
                };

                let aria_label = aria_label.clone();
                view! {
                    <div
                        class=overlay_classes
                        role="presentation"
                        on:click=handle_overlay_click
                        on:animationend=handle_animation_end
                    >
                        <div
                            class=drawer_classes
                            style=height.style()
                            role="dialog"
                            aria-modal="true"
                            aria-label=aria_label.clone()
                            on:click=stop_propagation
                            on:animationend=handle_animation_end
                        >
                            <Show when=move || show_handle>
                                <div class="ui-drawer__handle-section">
                                    <div class="ui-drawer__handle"></div>
                                </div>
                            </Show>
                            <div class="ui-drawer__content">
                                {children()}
                            </div>
                        </div>
                    </div>
                }
            }
        </Show>
    }
}

/// Drawer header with title and optional close button
#[component]
pub fn DrawerHeader(
    /// Title text
    #[prop(into)]
    title: String,
    /// Close button callback
    #[prop(optional)]
    on_close: Option<Callback<()>>,
    /// Subtitle text
    #[prop(optional, into)]
    subtitle: Option<String>
) -> impl IntoView {
    view! {
        <div class="ui-drawer__header">
            <div class="ui-drawer__header-text">
                <h2 class="ui-drawer__title">{title}</h2>
                {subtitle.map(|s| view! {
                    <p class="ui-drawer__subtitle">{s}</p>
                })}
            </div>
            {on_close.map(|cb| view! {
                <button
                    class="ui-drawer__close"
                    aria-label="Close"
                    on:click=move |_| cb.run(())
                >
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
                        <path d="M18 6L6 18M6 6L18 18" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
                    </svg>
                </button>
            })}
        </div>
    }
}

/// Drawer body with optional scrolling
#[component]
pub fn DrawerBody(
    /// Enable scrollable content
    #[prop(default = true)]
    scroll: bool,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: String,
    /// Body contents
    children: Children
) -> impl IntoView {
    let classes = {
        let mut classes = vec!["ui-drawer__body"];
        if scroll {
            classes.push("ui-drawer__body--scroll");
        }
        if !class.is_empty() {
            format!("{} {}", classes.join(" "), class)
        } else {
            classes.join(" ")
        }
    };

    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

/// Drawer footer for action buttons
#[component]
pub fn DrawerFooter(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: String,
    /// Footer contents
    children: Children
) -> impl IntoView {
    let classes = if class.is_empty() {
        "ui-drawer__footer".to_string()
    } else {
        format!("ui-drawer__footer {}", class)
    };

    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

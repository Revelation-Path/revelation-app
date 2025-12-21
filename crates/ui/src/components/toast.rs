//! Toast notification system

use leptos::prelude::*;

/// Toast notification variant
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ToastVariant {
    #[default]
    Info,
    Success,
    Warning,
    Error
}

impl ToastVariant {
    fn class(&self) -> &'static str {
        match self {
            Self::Info => "ui-toast--info",
            Self::Success => "ui-toast--success",
            Self::Warning => "ui-toast--warning",
            Self::Error => "ui-toast--error"
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            Self::Info => "i",
            Self::Success => "check",
            Self::Warning => "warning",
            Self::Error => "x"
        }
    }
}

/// Toast position
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ToastPosition {
    #[default]
    Top,
    TopRight,
    TopLeft,
    Bottom,
    BottomRight,
    BottomLeft
}

impl ToastPosition {
    fn class(&self) -> &'static str {
        match self {
            Self::Top => "ui-toast-container--top",
            Self::TopRight => "ui-toast-container--top-right",
            Self::TopLeft => "ui-toast-container--top-left",
            Self::Bottom => "ui-toast-container--bottom",
            Self::BottomRight => "ui-toast-container--bottom-right",
            Self::BottomLeft => "ui-toast-container--bottom-left"
        }
    }
}

/// Single toast data
#[derive(Clone, PartialEq)]
pub struct ToastData {
    pub id:          u32,
    pub message:     String,
    pub variant:     ToastVariant,
    pub duration_ms: u32
}

/// Toast notification component
#[component]
pub fn Toast(
    /// Toast message
    #[prop(into)]
    message: String,
    /// Toast variant
    #[prop(default = ToastVariant::Info)]
    variant: ToastVariant,
    /// Whether toast is exiting (for animation)
    #[prop(default = false)]
    exiting: bool,
    /// Dismiss callback
    #[prop(optional)]
    on_dismiss: Option<Callback<()>>
) -> impl IntoView {
    let handle_dismiss = move |_| {
        if let Some(cb) = on_dismiss {
            cb.run(());
        }
    };

    let classes = move || {
        let mut classes = vec!["ui-toast", variant.class()];
        if exiting {
            classes.push("ui-toast--exiting");
        }
        classes.join(" ")
    };

    view! {
        <div class=classes role="alert">
            <span class="ui-toast__icon" data-icon=variant.icon()></span>
            <span class="ui-toast__message">{message}</span>
            <button
                class="ui-toast__dismiss"
                aria-label="Dismiss"
                on:click=handle_dismiss
            >
                <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
                    <path d="M10.5 3.5L3.5 10.5M3.5 3.5L10.5 10.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
                </svg>
            </button>
        </div>
    }
}

/// Toast container for stacking notifications
#[component]
pub fn ToastContainer(
    /// Container position
    #[prop(default = ToastPosition::Top)]
    position: ToastPosition,
    /// Toast children
    children: Children
) -> impl IntoView {
    let classes = format!("ui-toast-container {}", position.class());

    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

/// Toast provider context
#[derive(Clone)]
pub struct ToastContext {
    toasts:  RwSignal<Vec<ToastData>>,
    next_id: RwSignal<u32>
}

impl ToastContext {
    /// Show a toast notification
    pub fn show(&self, message: impl Into<String>, variant: ToastVariant) {
        let id = self.next_id.get();
        self.next_id.set(id + 1);

        let toast = ToastData {
            id,
            message: message.into(),
            variant,
            duration_ms: 4000
        };

        self.toasts.update(|t| t.push(toast));

        // Auto-dismiss
        let toasts = self.toasts;
        set_timeout(
            move || {
                toasts.update(|t| t.retain(|toast| toast.id != id));
            },
            std::time::Duration::from_millis(4000)
        );
    }

    /// Show success toast
    pub fn success(&self, message: impl Into<String>) {
        self.show(message, ToastVariant::Success);
    }

    /// Show error toast
    pub fn error(&self, message: impl Into<String>) {
        self.show(message, ToastVariant::Error);
    }

    /// Show warning toast
    pub fn warning(&self, message: impl Into<String>) {
        self.show(message, ToastVariant::Warning);
    }

    /// Show info toast
    pub fn info(&self, message: impl Into<String>) {
        self.show(message, ToastVariant::Info);
    }

    /// Dismiss a specific toast
    pub fn dismiss(&self, id: u32) {
        self.toasts.update(|t| t.retain(|toast| toast.id != id));
    }
}

/// Toast provider component
#[component]
pub fn ToastProvider(
    /// Position of toasts
    #[prop(default = ToastPosition::Top)]
    position: ToastPosition,
    /// App children
    children: Children
) -> impl IntoView {
    let toasts = RwSignal::new(Vec::<ToastData>::new());
    let next_id = RwSignal::new(0u32);

    let context = ToastContext {
        toasts,
        next_id
    };

    provide_context(context.clone());

    view! {
        {children()}
        <ToastContainer position=position>
            <For
                each=move || toasts.get()
                key=|toast| toast.id
                children=move |toast| {
                    let ctx = context.clone();
                    let id = toast.id;
                    view! {
                        <Toast
                            message=toast.message.clone()
                            variant=toast.variant
                            on_dismiss=Callback::new(move |_| ctx.dismiss(id))
                        />
                    }
                }
            />
        </ToastContainer>
    }
}

/// Hook to access toast context
pub fn use_toast() -> ToastContext {
    expect_context::<ToastContext>()
}

//! Icon component with common icons

use leptos::prelude::*;

/// Icon size
#[derive(Clone, Copy, PartialEq, Default)]
pub enum IconSize {
    Small,
    #[default]
    Medium,
    Large,
    XLarge
}

impl IconSize {
    fn size(&self) -> u32 {
        match self {
            Self::Small => 16,
            Self::Medium => 20,
            Self::Large => 24,
            Self::XLarge => 32
        }
    }
}

/// Common icons
#[derive(Clone, Copy, PartialEq)]
pub enum IconType {
    // Navigation
    Home,
    Book,
    Search,
    Settings,
    User,
    Menu,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    ChevronDown,
    ArrowLeft,
    ArrowRight,

    // Actions
    Plus,
    Minus,
    Close,
    Check,
    Edit,
    Delete,
    Share,
    Download,
    Upload,
    Refresh,
    Copy,

    // Status
    Info,
    Warning,
    Error,
    Success,
    Heart,
    HeartFilled,
    Star,
    StarFilled,
    Bookmark,
    BookmarkFilled,

    // Bible/Spiritual
    Cross,
    Pray,
    Bible,
    Dove,
    Church,
    Candle,

    // Social
    Telegram,
    Share2,

    // Media
    Music
}

/// Icon component
#[component]
pub fn Icon(
    /// Icon type
    icon: IconType,
    /// Icon size
    #[prop(default = IconSize::Medium)]
    size: IconSize,
    /// Additional CSS classes
    #[prop(optional)]
    class: &'static str,
    /// Custom color
    #[prop(optional)]
    color: Option<&'static str>
) -> impl IntoView {
    let s = size.size();
    let style = color.map(|c| format!("color: {}", c));
    let classes = format!("ui-icon {}", class);

    let path = match icon {
        // Navigation
        IconType::Home => {
            "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
        }
        IconType::Book => {
            "M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
        }
        IconType::Search => "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z",
        IconType::Settings => {
            "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z M15 12a3 3 0 11-6 0 3 3 0 016 0z"
        }
        IconType::User => "M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z",
        IconType::Menu => "M4 6h16M4 12h16M4 18h16",
        IconType::ChevronLeft => "M15 19l-7-7 7-7",
        IconType::ChevronRight => "M9 5l7 7-7 7",
        IconType::ChevronUp => "M5 15l7-7 7 7",
        IconType::ChevronDown => "M19 9l-7 7-7-7",
        IconType::ArrowLeft => "M10 19l-7-7m0 0l7-7m-7 7h18",
        IconType::ArrowRight => "M14 5l7 7m0 0l-7 7m7-7H3",

        // Actions
        IconType::Plus => "M12 4v16m8-8H4",
        IconType::Minus => "M20 12H4",
        IconType::Close => "M6 18L18 6M6 6l12 12",
        IconType::Check => "M5 13l4 4L19 7",
        IconType::Edit => {
            "M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
        }
        IconType::Delete => {
            "M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
        }
        IconType::Share => {
            "M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z"
        }
        IconType::Download => "M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4",
        IconType::Upload => "M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12",
        IconType::Refresh => {
            "M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
        }
        IconType::Copy => {
            "M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"
        }

        // Status
        IconType::Info => "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
        IconType::Warning => {
            "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
        }
        IconType::Error => "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z",
        IconType::Success => "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z",
        IconType::Heart => {
            "M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"
        }
        IconType::HeartFilled => {
            "M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"
        }
        IconType::Star => {
            "M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z"
        }
        IconType::StarFilled => {
            "M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z"
        }
        IconType::Bookmark => "M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z",
        IconType::BookmarkFilled => "M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z",

        // Bible/Spiritual
        IconType::Cross => "M12 2v20M5 8h14",
        IconType::Pray => "M12 2v6m0 8v6M8 12H2m20 0h-6M7 7l3 3m4 4l3 3M17 7l-3 3m-4 4l-3 3",
        IconType::Bible => {
            "M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
        }
        IconType::Dove => "M12 19l-7-7 7-7M12 5l7 7-7 7",
        IconType::Church => "M12 2v4M8 6h8M4 22h16M7 22V10l5-4 5 4v12M10 22v-5h4v5",
        IconType::Candle => "M12 2v2M12 6v14M9 20h6M10 6h4v2c0 1-1 2-2 2s-2-1-2-2V6z",

        // Social
        IconType::Telegram => "M21 5L2 12.5l7 1M21 5l-12 8.5M21 5l-4 15-8-6.5",
        IconType::Share2 => {
            "M18 8a3 3 0 100-6 3 3 0 000 6zM6 15a3 3 0 100-6 3 3 0 000 6zM18 22a3 3 0 100-6 3 3 0 000 6zM8.59 13.51l6.83 3.98M15.41 6.51l-6.82 3.98"
        }

        // Media
        IconType::Music => "M9 18V5l12-2v13M9 18a3 3 0 11-6 0 3 3 0 016 0zM21 16a3 3 0 11-6 0 3 3 0 016 0z"
    };

    let filled = matches!(
        icon,
        IconType::HeartFilled | IconType::StarFilled | IconType::BookmarkFilled
    );

    view! {
        <svg
            class=classes
            style=style
            width=s
            height=s
            viewBox="0 0 24 24"
            fill=if filled { "currentColor" } else { "none" }
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d=path />
        </svg>
    }
}

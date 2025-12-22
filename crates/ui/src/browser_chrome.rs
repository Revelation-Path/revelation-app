//! Browser chrome color management
//!
//! Controls the browser's status bar color via meta theme-color tag.
//! Reads colors from CSS variables for consistency with the theme system.

/// Book category for color mapping
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum BookCategory {
    #[default]
    Torah,
    History,
    Wisdom,
    MajorProphets,
    MinorProphets,
    Gospels,
    Acts,
    Paul,
    General,
    Revelation
}

impl BookCategory {
    /// Get category from book ID
    pub fn from_book_id(book_id: i16) -> Self {
        match book_id {
            1..=5 => Self::Torah,
            6..=17 => Self::History,
            18..=22 => Self::Wisdom,
            23..=27 => Self::MajorProphets,
            28..=39 => Self::MinorProphets,
            40..=43 => Self::Gospels,
            44 => Self::Acts,
            45..=57 => Self::Paul,
            58..=65 => Self::General,
            66 => Self::Revelation,
            _ => Self::Gospels
        }
    }

    /// CSS variable name for this category
    pub fn css_var(&self) -> &'static str {
        match self {
            Self::Torah => "--cat-torah",
            Self::History => "--cat-history",
            Self::Wisdom => "--cat-wisdom",
            Self::MajorProphets => "--cat-major-prophets",
            Self::MinorProphets => "--cat-minor-prophets",
            Self::Gospels => "--cat-gospels",
            Self::Acts => "--cat-acts",
            Self::Paul => "--cat-paul",
            Self::General => "--cat-general",
            Self::Revelation => "--cat-revelation"
        }
    }
}

/// Browser chrome controller
pub struct BrowserChrome;

impl BrowserChrome {
    /// Set chrome color directly
    pub fn set_color(color: &str) {
        let Some(document) = web_sys::window().and_then(|w| w.document()) else {
            return;
        };

        if let Ok(Some(meta)) = document.query_selector("meta[name=\"theme-color\"]") {
            let _ = meta.set_attribute("content", color);
        }
    }

    /// Set chrome color from CSS variable
    pub fn set_from_var(var_name: &str) {
        let Some(color) = Self::get_css_var(var_name) else {
            return;
        };
        Self::set_color(&color);
    }

    /// Set chrome to book category color
    pub fn set_book_category(book_id: i16) {
        let category = BookCategory::from_book_id(book_id);
        Self::set_from_var(category.css_var());
    }

    /// Set chrome to theme background
    pub fn set_theme_bg() {
        Self::set_from_var("--bg");
    }

    /// Get computed CSS variable value
    fn get_css_var(var_name: &str) -> Option<String> {
        let window = web_sys::window()?;
        let document = window.document()?;
        let element = document.document_element()?;
        let style = window.get_computed_style(&element).ok()??;
        let value = style.get_property_value(var_name).ok()?;
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    }
}

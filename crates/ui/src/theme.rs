//! Theme system - dynamic theming with accent color calculation
//!
//! All colors are derived from a single accent hue via HSL rotation.
//! Category colors automatically adapt to the selected theme.
//!
//! Usage:
//! ```ignore
//! use ui::theme::ThemeProvider;
//!
//! view! {
//!     <ThemeProvider>
//!         // Your app content
//!     </ThemeProvider>
//! }
//! ```

use leptos::prelude::*;

/// Theme variants
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum Theme {
    #[default]
    Light,
    Dark,
    Sepia
}

impl Theme {
    /// Returns CSS class name for the theme
    pub fn class(&self) -> &'static str {
        match self {
            Theme::Light => "theme-light",
            Theme::Dark => "theme-dark",
            Theme::Sepia => "theme-sepia"
        }
    }

    /// Returns background color for browser chrome (theme-color meta)
    pub fn bg_color(&self) -> &'static str {
        match self {
            Theme::Light => "#fefdfb",
            Theme::Dark => "#1a1a1a",
            Theme::Sepia => "#f4ecd8"
        }
    }
}

/// Font family variants
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum FontFamily {
    #[default]
    Serif,
    Sans
}

impl FontFamily {
    /// Returns CSS class name for the font
    pub fn class(&self) -> &'static str {
        match self {
            FontFamily::Serif => "font-serif",
            FontFamily::Sans => "font-sans"
        }
    }
}

/// Global theme state
#[derive(Clone, Copy)]
pub struct ThemeState {
    pub theme:          RwSignal<Theme>,
    pub font_family:    RwSignal<FontFamily>,
    pub font_size:      RwSignal<u8>,
    pub verse_per_line: RwSignal<bool>
}

impl ThemeState {
    /// Creates new theme state with defaults
    pub fn new() -> Self {
        Self {
            theme:          RwSignal::new(Theme::Light),
            font_family:    RwSignal::new(FontFamily::Serif),
            font_size:      RwSignal::new(18),
            verse_per_line: RwSignal::new(false)
        }
    }

    /// Get CSS class string for current theme
    pub fn class(&self) -> String {
        format!(
            "{} {}",
            self.theme.get().class(),
            self.font_family.get().class()
        )
    }
}

impl Default for ThemeState {
    fn default() -> Self {
        Self::new()
    }
}

/// Theme provider component - wraps app and applies theme
#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let state = ThemeState::new();
    provide_context(state);

    let class = move || state.class();
    let font_size = move || format!("{}px", state.font_size.get());

    view! {
        <div class=class style:font-size=font_size>
            {children()}
        </div>
    }
}

/// Hook to access theme state
pub fn use_theme() -> ThemeState {
    expect_context::<ThemeState>()
}

/// CSS for theme system
pub const THEME_CSS: &str = r#"
/* ═══════════════════════════════════════════════════════════════════════════
   THEME SYSTEM - Dynamic theming with accent color calculation
   All colors derived from single accent via HSL hue rotation
   ═══════════════════════════════════════════════════════════════════════════ */

/* Base variables */
:root {
  /* Accent in HSL (gold: h=45, s=70%, l=47%) */
  --accent-h: 45;
  --accent-s: 70%;
  --accent-l: 47%;
  --accent: hsl(var(--accent-h), var(--accent-s), var(--accent-l));

  /* Fonts */
  --font-sans: 'Inter', system-ui, sans-serif;
  --font-serif: 'Merriweather', Georgia, serif;
}

/* ─────────────────────────────────────────────────────────────────────────────
   LIGHT THEME (default)
   ───────────────────────────────────────────────────────────────────────────── */
.theme-light {
  --bg: #fefdfb;
  --bg-secondary: #f8f6f3;
  --bg-elevated: #ffffff;
  --bg-overlay: rgba(0, 0, 0, 0.5);

  --text: #1f2937;
  --text-secondary: #4b5563;
  --text-muted: #9ca3af;
  --text-inverse: #ffffff;

  --border: rgba(0, 0, 0, 0.08);
  --hover: rgba(0, 0, 0, 0.04);
  --active: rgba(0, 0, 0, 0.08);
  --shadow: rgba(0, 0, 0, 0.1);

  --accent-soft: color-mix(in srgb, var(--accent) 15%, transparent);
  --accent-hover: color-mix(in srgb, var(--accent) 90%, black);
  --accent-text: var(--accent);

  /* Category colors - soft pastels for light theme */
  --cat-l: 78%;
  --cat-s: 35%;
  --cat-torah: hsl(calc(var(--accent-h) + 180), var(--cat-s), var(--cat-l));
  --cat-history: hsl(calc(var(--accent-h) + 120), var(--cat-s), var(--cat-l));
  --cat-wisdom: hsl(var(--accent-h), var(--cat-s), var(--cat-l));
  --cat-major-prophets: hsl(calc(var(--accent-h) + 315), var(--cat-s), var(--cat-l));
  --cat-minor-prophets: hsl(calc(var(--accent-h) + 270), var(--cat-s), var(--cat-l));
  --cat-gospels: hsl(calc(var(--accent-h) + 15), var(--cat-s), var(--cat-l));
  --cat-acts: hsl(calc(var(--accent-h) + 200), var(--cat-s), var(--cat-l));
  --cat-paul: hsl(calc(var(--accent-h) + 240), var(--cat-s), var(--cat-l));
  --cat-general: hsl(calc(var(--accent-h) + 150), var(--cat-s), var(--cat-l));
  --cat-revelation: hsl(calc(var(--accent-h) + 330), var(--cat-s), var(--cat-l));

  color-scheme: light;
}

/* ─────────────────────────────────────────────────────────────────────────────
   DARK THEME
   ───────────────────────────────────────────────────────────────────────────── */
.theme-dark {
  --bg: #1a1a1a;
  --bg-secondary: #242424;
  --bg-elevated: #2e2e2e;
  --bg-overlay: rgba(0, 0, 0, 0.6);

  --text: #d4d4d4;
  --text-secondary: #a0a0a0;
  --text-muted: #6b6b6b;
  --text-inverse: #1a1a1a;

  --border: rgba(255, 255, 255, 0.1);
  --hover: rgba(255, 255, 255, 0.05);
  --active: rgba(255, 255, 255, 0.1);
  --shadow: rgba(0, 0, 0, 0.4);

  --accent-soft: color-mix(in srgb, var(--accent) 20%, transparent);
  --accent-hover: color-mix(in srgb, var(--accent) 80%, white);
  --accent-text: color-mix(in srgb, var(--accent) 80%, white);

  /* Category colors - muted but visible on dark bg */
  --cat-l: 45%;
  --cat-s: 30%;
  --cat-torah: hsl(calc(var(--accent-h) + 180), var(--cat-s), var(--cat-l));
  --cat-history: hsl(calc(var(--accent-h) + 120), var(--cat-s), var(--cat-l));
  --cat-wisdom: hsl(var(--accent-h), var(--cat-s), var(--cat-l));
  --cat-major-prophets: hsl(calc(var(--accent-h) + 315), var(--cat-s), var(--cat-l));
  --cat-minor-prophets: hsl(calc(var(--accent-h) + 270), var(--cat-s), var(--cat-l));
  --cat-gospels: hsl(calc(var(--accent-h) + 15), var(--cat-s), var(--cat-l));
  --cat-acts: hsl(calc(var(--accent-h) + 200), var(--cat-s), var(--cat-l));
  --cat-paul: hsl(calc(var(--accent-h) + 240), var(--cat-s), var(--cat-l));
  --cat-general: hsl(calc(var(--accent-h) + 150), var(--cat-s), var(--cat-l));
  --cat-revelation: hsl(calc(var(--accent-h) + 330), var(--cat-s), var(--cat-l));

  color-scheme: dark;
}

/* ─────────────────────────────────────────────────────────────────────────────
   SEPIA THEME
   ───────────────────────────────────────────────────────────────────────────── */
.theme-sepia {
  --bg: #f4ecd8;
  --bg-secondary: #e8dcc4;
  --bg-elevated: #faf6ed;
  --bg-overlay: rgba(67, 52, 34, 0.5);

  --text: #433422;
  --text-secondary: #5c4a35;
  --text-muted: #8b7355;
  --text-inverse: #faf6ed;

  --border: rgba(67, 52, 34, 0.12);
  --hover: rgba(67, 52, 34, 0.06);
  --active: rgba(67, 52, 34, 0.12);
  --shadow: rgba(67, 52, 34, 0.15);

  --accent-soft: color-mix(in srgb, var(--accent) 12%, transparent);
  --accent-hover: color-mix(in srgb, var(--accent) 85%, #433422);
  --accent-text: color-mix(in srgb, var(--accent) 70%, #433422);

  /* Category colors - warm, desaturated for sepia */
  --cat-l: 62%;
  --cat-s: 25%;
  --cat-torah: hsl(calc(var(--accent-h) + 180), var(--cat-s), var(--cat-l));
  --cat-history: hsl(calc(var(--accent-h) + 120), var(--cat-s), var(--cat-l));
  --cat-wisdom: hsl(var(--accent-h), var(--cat-s), var(--cat-l));
  --cat-major-prophets: hsl(calc(var(--accent-h) + 315), var(--cat-s), var(--cat-l));
  --cat-minor-prophets: hsl(calc(var(--accent-h) + 270), var(--cat-s), var(--cat-l));
  --cat-gospels: hsl(calc(var(--accent-h) + 15), var(--cat-s), var(--cat-l));
  --cat-acts: hsl(calc(var(--accent-h) + 200), var(--cat-s), var(--cat-l));
  --cat-paul: hsl(calc(var(--accent-h) + 240), var(--cat-s), var(--cat-l));
  --cat-general: hsl(calc(var(--accent-h) + 150), var(--cat-s), var(--cat-l));
  --cat-revelation: hsl(calc(var(--accent-h) + 330), var(--cat-s), var(--cat-l));

  color-scheme: light;
}

/* ─────────────────────────────────────────────────────────────────────────────
   FONT FAMILIES
   ───────────────────────────────────────────────────────────────────────────── */
.font-serif { font-family: var(--font-serif); }
.font-sans { font-family: var(--font-sans); }

/* ─────────────────────────────────────────────────────────────────────────────
   BASE STYLES
   ───────────────────────────────────────────────────────────────────────────── */
.theme-light,
.theme-dark,
.theme-sepia {
  background: var(--bg);
  color: var(--text);
  transition: background 0.2s ease, color 0.2s ease;
  min-height: 100dvh;
}

/* ─────────────────────────────────────────────────────────────────────────────
   UTILITY CLASSES
   ───────────────────────────────────────────────────────────────────────────── */

/* Backgrounds */
.bg-primary { background: var(--bg); }
.bg-secondary { background: var(--bg-secondary); }
.bg-elevated { background: var(--bg-elevated); }
.bg-accent { background: var(--accent); }
.bg-accent-soft { background: var(--accent-soft); }

/* Text colors */
.text-primary { color: var(--text); }
.text-secondary { color: var(--text-secondary); }
.text-muted { color: var(--text-muted); }
.text-accent { color: var(--accent-text); }
.text-inverse { color: var(--text-inverse); }

/* Borders */
.border-default { border-color: var(--border); }
.border-accent { border-color: var(--accent); }

/* Interactive states */
.hover\:bg-hover:hover { background: var(--hover); }
.hover\:bg-active:active { background: var(--active); }
.hover\:bg-accent:hover { background: var(--accent); }

/* Accent button */
.btn-accent {
  background: var(--accent);
  color: var(--text-inverse);
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 0.5rem;
  font-weight: 500;
  cursor: pointer;
  transition: filter 0.15s ease, transform 0.15s ease;
}

.btn-accent:hover {
  filter: brightness(1.1);
  transform: translateY(-1px);
}

.btn-accent:active {
  filter: brightness(0.95);
  transform: translateY(0);
}

/* Ghost button */
.btn-ghost {
  background: transparent;
  color: var(--text-secondary);
  border: 1px solid var(--border);
  padding: 0.5rem 1rem;
  border-radius: 0.5rem;
  cursor: pointer;
  transition: background 0.15s ease, color 0.15s ease;
}

.btn-ghost:hover {
  background: var(--hover);
  color: var(--text);
}

/* Card */
.card {
  background: var(--bg-elevated);
  border: 1px solid var(--border);
  border-radius: 0.75rem;
  box-shadow: 0 1px 3px var(--shadow);
}

/* Input */
.input {
  background: var(--bg);
  color: var(--text);
  border: 1px solid var(--border);
  padding: 0.5rem 0.75rem;
  border-radius: 0.5rem;
  transition: border-color 0.15s ease;
}

.input:focus {
  outline: none;
  border-color: var(--accent);
}

.input::placeholder {
  color: var(--text-muted);
}
"#;

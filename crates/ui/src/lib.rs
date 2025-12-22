//! Revelation UI Component Library
//!
//! Professional, accessible, Christian-themed UI components for Leptos.

#![deny(rust_2018_idioms)]

pub mod browser_chrome;
pub mod components;
pub mod theme;

pub use browser_chrome::{BookCategory, BrowserChrome};
pub use components::*;
pub use theme::{FontFamily, THEME_CSS, Theme, ThemeProvider, ThemeState, use_theme};

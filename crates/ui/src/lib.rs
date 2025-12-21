//! Revelation UI Component Library
//!
//! Professional, accessible, Christian-themed UI components for Leptos.

#![deny(rust_2018_idioms)]

pub mod components;
pub mod theme;

pub use components::*;
pub use theme::{FontFamily, THEME_CSS, Theme, ThemeProvider, ThemeState, use_theme};

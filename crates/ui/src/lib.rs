//! Revelation UI Component Library
//!
//! Professional, accessible, Christian-themed UI components for Leptos.

#![deny(rust_2018_idioms)]

pub mod components;

pub use components::*;

/// CSS styles for all UI components
pub const STYLES: &str = include_str!("styles.css");

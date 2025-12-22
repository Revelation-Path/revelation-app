//! Haptic feedback manager - vibration support detection and control

use wasm_bindgen::prelude::*;
use web_sys::js_sys;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = navigator, js_name = vibrate)]
    fn navigator_vibrate(pattern: u32) -> bool;

    #[wasm_bindgen(js_namespace = navigator, js_name = vibrate)]
    fn navigator_vibrate_pattern(pattern: &js_sys::Array) -> bool;
}

/// Check if vibration API is supported
pub fn is_supported() -> bool {
    web_sys::window().is_some()
}

/// Light tap feedback (10ms)
pub fn tap() {
    let _ = navigator_vibrate(10);
}

/// Medium feedback (25ms)
pub fn medium() {
    let _ = navigator_vibrate(25);
}

/// Heavy feedback (50ms)
pub fn heavy() {
    let _ = navigator_vibrate(50);
}

/// Success pattern (short-pause-short)
pub fn success() {
    let pattern = js_sys::Array::new();
    pattern.push(&JsValue::from(10));
    pattern.push(&JsValue::from(50));
    pattern.push(&JsValue::from(10));
    let _ = navigator_vibrate_pattern(&pattern);
}

/// Error pattern (long-pause-long-pause-long)
pub fn error() {
    let pattern = js_sys::Array::new();
    pattern.push(&JsValue::from(50));
    pattern.push(&JsValue::from(100));
    pattern.push(&JsValue::from(50));
    pattern.push(&JsValue::from(100));
    pattern.push(&JsValue::from(50));
    let _ = navigator_vibrate_pattern(&pattern);
}

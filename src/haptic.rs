//! Haptic feedback manager - vibration support detection and control

use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = "
export function vibrate(ms) {
    if (navigator.vibrate) {
        navigator.vibrate(ms);
    }
}

export function vibratePattern(pattern) {
    if (navigator.vibrate) {
        navigator.vibrate(pattern);
    }
}

export function hasVibrate() {
    return typeof navigator.vibrate === 'function';
}
")]
extern "C" {
    fn vibrate(ms: u32);
    fn vibratePattern(pattern: &[u32]);
    fn hasVibrate() -> bool;
}

/// Check if vibration API is supported.
#[must_use]
pub fn is_supported() -> bool {
    hasVibrate()
}

/// Light tap feedback (10ms)
pub fn tap() {
    vibrate(10);
}

/// Medium feedback (25ms)
pub fn medium() {
    vibrate(25);
}

/// Heavy feedback (50ms)
pub fn heavy() {
    vibrate(50);
}

/// Success pattern (short-pause-short)
pub fn success() {
    vibratePattern(&[10, 50, 10]);
}

/// Error pattern (long-pause-long-pause-long)
pub fn error() {
    vibratePattern(&[50, 100, 50, 100, 50]);
}

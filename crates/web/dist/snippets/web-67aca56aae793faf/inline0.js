
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

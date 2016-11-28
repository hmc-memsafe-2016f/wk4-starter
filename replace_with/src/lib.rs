use std::ptr;

/// Replaces `*t` with `f` applied to the original `*t`,
pub fn replace_with<T, F: FnOnce(T) -> T>(t: &mut T, f: F) {
    unsafe {
        *t = f(ptr::read(t));
    }
}
use std::ptr;

/// Replaces `*t` with `f` applied to the original `*t`,
pub fn replace_with<T, F: FnOnce(T) -> T>(t: &mut T, f: F) {
    let pointer = t as *mut T;
    unsafe { ptr::write(pointer, f(ptr::read(pointer))) }
}

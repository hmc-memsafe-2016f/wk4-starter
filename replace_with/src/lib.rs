use std::mem;

/// Replaces `*t` with `f` applied to the original contents of `t`,
pub fn replace_with<T, F: FnOnce(T) -> T>(t: &mut T, f: F) {
    unsafe {
        let new = f(mem::replace(t, mem::uninitialized()));
        mem::forget(mem::replace(t, new))
    }
}

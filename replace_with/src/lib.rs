/// Replaces `*t` with `f` applied to the original `*t`,
pub fn replace_with<T, F: FnOnce(T) -> T>(t: &mut T, f: F) {
    let raw = t as *mut T;
    unsafe{
        std::ptr::write(raw, f(std::ptr::read(raw)));
    }
}
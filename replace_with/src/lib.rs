/// Replaces `*t` with `f` applied to the original `*t`,
pub fn replace_with<T, F: FnOnce(T) -> T>(t: &mut T, f: F)
{
    let mut ptr: *mut T = t;
    unsafe {
        *ptr = f(std::ptr::read(ptr));
    }
}
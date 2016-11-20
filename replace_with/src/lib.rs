use std::mem;

pub fn replace_with<T, F: FnOnce(T) -> T>(t: &mut T, f: F)
{
    unsafe {
        let old = mem::replace(t, mem::uninitialized());
        let garbage = mem::replace(t, f(old));
        mem::forget(garbage);
    }
}

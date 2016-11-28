use std::ptr;

pub fn replace_with<T, F: FnOnce(T) -> T>(t: &mut T, f: F) {
    let p = t as *mut T;
    unsafe {
        ptr::write(p, f(ptr::read(p)))
    }
}

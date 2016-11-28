use std::ptr;
use std::fmt;

pub fn replace_with<T: fmt::Debug, F: FnOnce(T) -> T>(t : &mut T, f: F) {
    let t_ptr = t as *mut T;
    unsafe { 
        let new_int = ptr::read(t_ptr);
        ptr::write(t_ptr, f(new_int));
    }
}


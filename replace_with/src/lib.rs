use std::mem;
use std::ptr;

pub fn replace_with<T, F: FnOnce(T) -> T>(t: &mut T, f: F) {
    unsafe {
        let mut scratch: T = mem::uninitialized();
        ptr::copy_nonoverlapping(t, &mut scratch, 1);
        scratch = f(scratch);
        ptr::copy_nonoverlapping(&mut scratch, t, 1);
        mem::forget(scratch);
    }
}


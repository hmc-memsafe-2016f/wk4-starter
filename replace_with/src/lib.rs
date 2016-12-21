use std::ptr;

pub fn replace_with<T, F: FnOnce(T) -> T>(t: &mut T, f: F){
	unsafe{let mut new_val = ptr::read(t);
		   new_val = f(new_val);
		   ptr::write(t, new_val)}
}

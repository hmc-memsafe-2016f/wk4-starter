use std::ptr;

pub fn replace_with<T, F>(t: &mut T, f: F) where F: FnOnce(T) -> T
{
	unsafe
	{
		let t_ptr = t as *mut T;
		ptr::write(t, f(ptr::read(t_ptr)));
	}	
}
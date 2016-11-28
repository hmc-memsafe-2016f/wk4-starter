// Julien Chien <jchien17@cmc.edu>

pub fn replace_with<T, F: FnOnce(T) -> T>(t: &mut T, f: F) {
  unsafe {
    let mut ptr: *mut T = t;
    *ptr = f(std::ptr::read(ptr))
  }
}

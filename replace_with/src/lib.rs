// Julien Chien <jchien17@cmc.edu>

pub fn replace_with<T, F: FnOnce(T) -> T>(t: &mut T, f: F);

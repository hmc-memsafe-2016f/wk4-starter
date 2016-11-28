//Julien Chien <jchien17@cmc.edu>

use std::mem;
use std::ops::Deref;

pub struct MyRc<T> {
  ptr: *mut Data<T>
}

struct Data<T> {
  data: T,
  count: usize
}

impl<T> MyRc<T> {
  pub fn new(t: T) -> MyRc<T> {
    let data = Data{ data: t, count: 1 };
    MyRc{ ptr: Box::into_raw(Box::new(data)) }
  }

  pub fn consume(self) -> Result<T, MyRc<T>> {
    unsafe {
      if (*self.ptr).count == 1 {
        let uninit: Data<T> = mem::uninitialized();
        let t = std::ptr::replace(self.ptr, uninit);
        mem::forget(self);

        Ok(t.data)

      } else {
        Err(self)
      }
    }
  }
}

impl<T> Deref for MyRc<T> { // refer to the underlying data
   type Target = T;

   fn deref(&self) -> &T {
     unsafe {
       &(*self.ptr).data
     }
   }
}

impl<T> Clone for MyRc<T> { // Make a new `MyRc` to the same underlying data
  fn clone (&self) -> Self {
    unsafe {
      (*self.ptr).count += 1
    }
    MyRc{ptr: self.ptr}
  }
}

impl<T> Drop for MyRc<T> {
  fn drop(&mut self) {
    unsafe {
      if (*self.ptr).count == 1 {
        //unused variable warning for lol
        let lol = Box::from_raw(self.ptr);
      } else {
        (*self.ptr).count -= 1
      }
    }
  }
}


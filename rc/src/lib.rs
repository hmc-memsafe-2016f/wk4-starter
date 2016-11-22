//Julien Chien <jchien17@cmc.edu>

pub struct MyRc<T> {
  ptr: *mut Data<t>
}

struct Data<T> {
  data: T,
  count: usize
}

impl<T> MyRc<T> {
  pub fn new(t: T) -> MyRc<T> {
    let data = Data{ data: t, count: 1 };
    MyRc{ ptr: Box::into_raw(data) }
  }

  pub fn consume(self) -> Result<T, MyRc<T>> {
    unsafe {
      if (*self.ptr).count == 1 {
        let val = std::ptr::read(self.data);
        let uninit: Data<T> = mem::uninitialized();
        let t = std::ptr::replace(self.ptr, uninit);
        mem::forget(self);

        Ok(t.data)

      } else {
        Err(self)
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

impl<T> Drop for MyRc<T> {
  fn drop(&mut self) {
    unsafe {
      if (*self.ptr).count == 1 {
        Box::from_raw(self.ptr)
      } else {
        (*self.ptr).count -= 1
      }
    }
  }
}


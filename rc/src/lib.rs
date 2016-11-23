use std::ops::Deref;
use std::ptr;

pub struct MyRc<T> {
  data: *mut T,
  count: *mut usize,
}

impl<T> MyRc<T> {
  pub fn new(t: T) -> MyRc<T> {
      let data_box  = Box::new(t);
      let count_box = Box::new(1);  // one reference to the underlying data
      let data_ptr  = Box::into_raw(data_box);  // now we own the heap data
      let count_ptr = Box::into_raw(count_box);
      MyRc{data: data_ptr, count: count_ptr}
  }

  pub fn consume(self) -> Result<T, MyRc<T> > {
    unsafe {
      if *self.count == 1 {
        let t: T = ptr::read(self.data);
        Ok(t)
      } else {
        Err(self)
      }
    }
  }
}

impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {
          &(*self.data)
        }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> MyRc<T> {
        unsafe {
          *self.count += 1;  // there is now another reference
          MyRc{data: self.data, count: self.count}
        }
    }
}

impl<T> Drop for MyRc<T> {
  fn drop(&mut self) {
    unsafe {
      if *self.count == 1 {
        Box::from_raw(self.data);   // convert back to boxes
        Box::from_raw(self.count);  // so destructors will run
      } else {
        *self.count -= 1;
      }
    }
  }
}


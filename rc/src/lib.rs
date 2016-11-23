use std::ops::Deref;
use std::ptr;

pub struct MyRc<T> {
    ptr: *mut T,
    count: *mut usize,
}

impl<T> Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { & *self.ptr }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> MyRc<T> {
        unsafe { *self.count += 1; }
        MyRc { ptr: self.ptr, count: self.count }
    }
}

impl<T> MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            *self.count -= 1;

            if *self.count == 0 {
                ptr::drop_in_place(self.ptr);
                ptr::drop_in_place(self.count);
            }
        }
    }

    pub fn new(t: T) -> MyRc<T> {
        let ptr_box = Box::new(t);
        let count_box = Box::new(1);
        MyRc { ptr: Box::into_raw(ptr_box), count: Box::into_raw(count_box) }
    }

    pub fn consume(self) -> Result<T, MyRc<T>> {
        unsafe {
            if *self.count == 1 {
                return Ok(ptr::read(self.ptr))
            }
        }
        Err(self)
    }
}

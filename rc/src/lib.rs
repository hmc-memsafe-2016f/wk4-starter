use std::ops::{Deref,Drop};
use std::{mem,ptr};

pub struct MyRc<T> {
    ptr: *mut RcT<T>
}

struct RcT<T> {
    data: T,
    count: usize
}

impl<T> MyRc<T> {

    pub fn new(t : T) -> MyRc<T> {
        let data = RcT { data: t, count: 1 };
        MyRc { ptr: Box::into_raw(Box::new(data)) }
    }

    pub fn consume(self) -> Result<T,MyRc<T>> {
        unsafe {
            if (*self.ptr).count == 1 {
                let mut t : T = mem::uninitialized();
                mem::swap(&mut (*self.ptr).data, &mut t);
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
        unsafe { &(*self.ptr).data }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        unsafe {
            (*self.ptr).count += 1;
            MyRc { ptr : self.ptr }
        }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            if (*self.ptr).count == 1 {
                Box::from_raw(self.ptr);
            } else {
                (*self.ptr).count -= 1
            }
        }
    }
}
use std::cell::Cell;
use std::ops::Deref;
use std::fmt::{self,Debug};
use std::mem;

pub struct MyRc<T> {
    ptr: *mut RcHeap<T>
}

struct RcHeap<T> {
    strong_count: Cell<usize>,
    data: T,
}

impl<T> MyRc<T> {
    pub fn new(t: T) -> Self {
        MyRc {
            ptr: Box::into_raw(Box::new(RcHeap {
                 strong_count: Cell::new(1usize),
                 data: t,
            }))
        }
    }
    pub fn consume(self) -> Result<T,Self> {
        unsafe {
            if (*self.ptr).strong_count.get() == 1 {
                self.dec_strong();
                let res = Ok(Box::from_raw(self.ptr).data);
                mem::forget(self);
                res
            } else {
                Err(self)
            }
        }
    }
    fn inc_strong(&self) {
        unsafe {
            (*self.ptr).inc_strong()
        }
    }
    fn dec_strong(&self) {
        unsafe {
            (*self.ptr).dec_strong()
        }
    }
}

impl<T> RcHeap<T> {
    fn inc_strong(&self) {
        self.strong_count.set(self.strong_count.get() + 1)
    }
    fn dec_strong(&self) {
        self.strong_count.set(self.strong_count.get() - 1)
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {
            & (*self.ptr).data
        }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        self.inc_strong();
        MyRc{ ptr: self.ptr }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            self.dec_strong();
            if (*self.ptr).strong_count.get() == 0 {
                Box::from_raw(self.ptr);
            }
        }
    }
}

impl<T: Debug> Debug for MyRc<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(& **self, f)
    }
}

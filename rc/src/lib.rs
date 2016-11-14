use std::cell::Cell;
use std::ops::Deref;
use std::fmt::{self,Debug};
use std::mem;

pub struct MyRc<T> {
    ptr: *mut RcHeap<T>
}

pub struct MyWeak<T> {
    ptr: *mut RcHeap<T>
}

struct RcHeap<T> {
    strong_count: Cell<usize>,
    weak_count: Cell<usize>,
    // This is not the right way to do this, but it allows use to avoid using allocators directly.
    // The model we're using for this assignment is that all allocation is done with boxes and
    // deallocation is done with boxes, permenantly entagling deallocation and destrcutors.
    data: *mut T,
}

impl<T> MyRc<T> {
    pub fn new(t: T) -> Self {
        MyRc {
            ptr: Box::into_raw(Box::new(RcHeap {
                 strong_count: Cell::new(1usize),
                 weak_count: Cell::new(0usize),
                 data: Box::into_raw(Box::new(t)),
            }))
        }
    }
    pub fn consume(self) -> Result<T,Self> {
        unsafe {
            if (*self.ptr).strong_count.get() == 1 {
                self.dec_strong();
                let res = Ok(*Box::from_raw((*self.ptr).data));
                if (*self.ptr).weak_count.get() == 0 {
                    Box::from_raw(self.ptr);
                }
                mem::forget(self);
                res
            } else {
                Err(self)
            }
        }
    }
    pub fn downgrade(&self) -> MyWeak<T> {
        MyWeak::new(self.ptr)
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
    fn inc_weak(&self) {
        self.weak_count.set(self.weak_count.get() + 1)
    }
    fn dec_weak(&self) {
        self.weak_count.set(self.weak_count.get() - 1)
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {
            & *(*self.ptr).data
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
                Box::from_raw((*self.ptr).data);
                if (*self.ptr).weak_count.get() == 0 {
                    Box::from_raw(self.ptr);
                }
            }
        }
    }
}

impl<T: Debug> Debug for MyRc<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(& **self, f)
    }
}

impl<T> MyWeak<T> {
    fn new(ptr: *mut RcHeap<T>) -> Self {
        let this = MyWeak{ ptr: ptr };
        this.inc_weak();
        this
    }
    fn inc_weak(&self) {
        unsafe {
            (*self.ptr).inc_weak()
        }
    }
    fn dec_weak(&self) {
        unsafe {
            (*self.ptr).dec_weak()
        }
    }
    pub fn upgrade(&self) -> Option<MyRc<T>> {
        unsafe {
            if (*self.ptr).strong_count.get() > 0 {
                let this = MyRc { ptr: self.ptr };
                this.inc_strong();
                Some(this)
            } else {
                None
            }
        }
    }
}

impl<T> Drop for MyWeak<T> {
    fn drop(&mut self) {
        unsafe {
            self.dec_weak();
            if (*self.ptr).strong_count.get() == 0 && (*self.ptr).weak_count.get() == 0 {
                Box::from_raw(self.ptr);
            }
        }
    }
}

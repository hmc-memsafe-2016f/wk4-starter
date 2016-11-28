use std::ptr;
use std::mem;
use std::clone::Clone;
use std::ops::{Deref,Drop};

// the heap object
struct RcCore<T> {
    count: usize,
    wcount: usize, // number of weakrefs
    val: Option<T>
}

// the reference wrapper to that heap object
pub struct MyRc<T> {
    core: *mut RcCore<T>
}

// weak ref
pub struct MyWeak<T> {
    core: *mut RcCore<T>
}

impl<T> RcCore<T> {
    fn new(t: T) -> RcCore<T> {
        RcCore::<T> { count: 1, wcount: 0, val: Some(t) }
    }
}

impl<T> MyRc<T> {
    pub fn new(t: T) -> MyRc<T> {
        MyRc::<T>{ core: Box::into_raw(Box::new(RcCore::<T>::new(t))) }
    }

    fn count(&self) -> usize {
        unsafe { (&*self.core).count }
    }

    fn decref(&self) -> usize {
        let core = unsafe { &mut *self.core };
        core.count = core.count - 1;
        core.count
    }

    fn incref(&self) {
        let core = unsafe { &mut *self.core };
        core.count = core.count + 1;
    }

    fn incref_weak(&self) {
        let core = unsafe { &mut *self.core };
        core.wcount = core.wcount + 1;
    }

    pub fn consume(self) -> Result<T,MyRc<T>> {
        match self.count() {
            1 => {
                let core = unsafe { &mut *self.core };
                Ok(core.val.take().unwrap())
            },
            _ => Err(self)
        }
    }

    pub fn downgrade(&self) -> MyWeak<T> {
        self.incref_weak();
        MyWeak::<T>{ core: self.core }
    }
}

impl<T> MyWeak<T> {
    fn decref_weak(&self) -> usize {
        let core = unsafe { &mut *self.core };
        core.wcount = core.wcount - 1;
        core.wcount
    }

    // stong count
    fn get_count(&self) -> usize {
        (unsafe { &*self.core }).count
    }

    fn incref(&self) {
        let core = unsafe { &mut *self.core };
        core.count = core.count + 1;
    }

    pub fn upgrade(&self) -> Option<MyRc<T>> {
        if self.get_count() != 0 {
            self.incref();
            Some(MyRc::<T>{ core: self.core })
        } else {
            None
        }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        // kill the T, if there is one
        if self.decref() == 0 {
            let core = unsafe { &mut *self.core };
            core.val.take();

            // free the heap object if we can
            if core.wcount == 0 {
                unsafe { Box::from_raw(self.core) };
            }
        }
    }
}

impl<T> Drop for MyWeak<T> {
    fn drop(&mut self) {
        // we are the last weak ref, and there are no more real refs
        if self.decref_weak() == 0 && self.get_count() == 0 {
            unsafe { Box::from_raw(self.core) };
        }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        let core = unsafe { &*self.core };
        core.val.as_ref().unwrap()
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        self.incref();
        MyRc::<T>{ core: self.core }
    }
}

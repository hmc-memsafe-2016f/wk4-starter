use std::ops::Deref;

pub struct MyRc<T> {
    ptr: *mut T,
    refcount: *mut usize,
}
impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        match unsafe{*self.refcount} {
            1 => unsafe {
                    Box::from_raw(self.ptr);
                    Box::from_raw(self.refcount);
                },
            _ => unsafe{std::ptr::write(self.refcount, *self.refcount - 1)},
        }
    }
}

impl<T> Deref for MyRc<T> { // refer to the underlying data
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}
impl<T> Clone for MyRc<T> { // Make a new `MyRc` to the same underlying data
    fn clone(&self) -> Self {
        unsafe{std::ptr::write(self.refcount, *self.refcount + 1)}
        MyRc {
            ptr: self.ptr,
            refcount: self.refcount,
        }
    }
}

impl<T> MyRc<T> {
    pub fn new(t: T) -> Self {
        MyRc {
            ptr: Box::into_raw(Box::new(t)),
            refcount: Box::into_raw(Box::new(1)),
        }
    }
    
    pub fn consume(self) -> Result<T,MyRc<T>> {
        match unsafe{*self.refcount} {
            1 => {
                    let ret = Ok(unsafe{*Box::from_raw(self.ptr)});
                    std::mem::forget(self);
                    ret
                },
            _ => Err(self),
        }
    }
}
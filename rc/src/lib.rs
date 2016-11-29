use std::ops::Deref;

pub struct MyRc<T> {
    ptr: *mut T,
    refcount: *mut usize,
    weakcount: *mut usize,
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        match unsafe{*self.refcount} {
            1 => {
                    unsafe{Box::from_raw(self.ptr);}
                    match unsafe{*self.weakcount} {
                        0 => {
                            unsafe{Box::from_raw(self.refcount);}
                            unsafe{Box::from_raw(self.weakcount);}
                        },
                        _ => unsafe{std::ptr::write(self.refcount, *self.refcount - 1);},
                    }
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
            weakcount: self.weakcount,
        }
    }
}

impl<T> MyRc<T> {
    pub fn new(t: T) -> Self {
        MyRc {
            ptr: Box::into_raw(Box::new(t)),
            refcount: Box::into_raw(Box::new(1)),
            weakcount: Box::into_raw(Box::new(0)),
        }
    }
    
    pub fn consume(self) -> Result<T,MyRc<T>> {
        match unsafe{*self.refcount} {
            1 => {
                    match unsafe{*self.weakcount} {
                        0 => {
                            unsafe{Box::from_raw(self.refcount);}
                            unsafe{Box::from_raw(self.weakcount);}
                        },
                        _ => unsafe{std::ptr::write(self.refcount, *self.refcount - 1);},
                    }
                    let ret = Ok(unsafe{*Box::from_raw(self.ptr)});
                    std::mem::forget(self);
                    ret
                },
            _ => Err(self),
        }
    }
    
    pub fn downgrade(&self) -> MyWeak<T> {
        unsafe{std::ptr::write(self.weakcount, *self.weakcount + 1)}
        MyWeak {
            rc: self as *const MyRc<T>,
        }
    }
}


pub struct MyWeak<T> {
    rc: *const MyRc<T>,
}

impl<T> MyWeak<T> {
    pub fn upgrade(&self) -> Option<MyRc<T>> {
        match unsafe{*(*self.rc).refcount} {
            0 => None,
            _ => unsafe{Some((*self.rc).clone())},
        }
    }
}

impl<T> Drop for MyWeak<T> {
    fn drop(&mut self) {
        match unsafe{*(*self.rc).refcount} {
            0 => {
                    match unsafe{*(*self.rc).weakcount} {
                        1 => {
                            unsafe{Box::from_raw((*self.rc).refcount);}
                            unsafe{Box::from_raw((*self.rc).weakcount);}
                        },
                        _ => unsafe{std::ptr::write((*self.rc).weakcount, *(*self.rc).weakcount - 1);},
                    }
                },
            _ => unsafe{std::ptr::write((*self.rc).weakcount, *(*self.rc).weakcount - 1);},
        }
    }
}
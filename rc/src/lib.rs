use std::ops;
use std::cell::Cell;

pub struct MyRc<T> {
    data: *mut MyBox<T>,
}

pub struct MyWeak<T> {
    data: *mut MyBox<T>,
}

struct MyBox<T> {
    data: *mut T,
    strong: Cell<usize>,
    weak: Cell<usize>
}

impl<T> MyRc<T> {
    pub fn new(t: T) -> Self {
        MyRc{ data: Box::into_raw(Box::new(
            MyBox{ data: Box::into_raw(Box::new(t)),
                   strong: Cell::new(1),
                   weak: Cell::new(0),
            }))
        }
    }

    pub fn consume(self) -> Result<T, MyRc<T>> {
        unsafe {
            if (*self.data).strong.get() == 1 {
                (*self.data).strong.set(0);
                Ok(*Box::from_raw((*self.data).data))
            } else {
                Err(self)
            }
        }
    }

    pub fn downgrade(&self) -> MyWeak<T> {
        unsafe {
            (*self.data).weak.set((*self.data).weak.get() + 1);
        }
        MyWeak{ data: self.data }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            // Last strong reference
            if (*self.data).strong.get() == 1 {
                Box::from_raw((*self.data).data);
            } 
            
            // Decrement strong count as long as it doesn't lead to underflow
            if (*self.data).strong.get() > 0 {
                (*self.data).strong.set((*self.data).strong.get() - 1);
            }

            // If we're out of both strong and weak references,
            // then delete the whole data structure, not just the
            // data inside of it

            if (*self.data).strong.get() == 0 && (*self.data).weak.get() == 0 {
                Box::from_raw(self.data);
            }
        } 
    }
}

impl<T> ops::Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(*self.data).data }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        unsafe {
            (*self.data).strong.set((*self.data).strong.get() + 1);
            MyRc{ data: self.data }
        }
    }
}

impl<T> MyWeak<T> {
    pub fn upgrade(&self) -> Option<MyRc<T>> {
        unsafe {
            if (*self.data).strong.get() > 0 {
                (*self.data).strong.set((*self.data).strong.get() + 1);
                Some(MyRc{ data: self.data })
            } else {
                None
            }
        }
    }
}


impl<T> Drop for MyWeak<T> {
    fn drop(&mut self) {
        unsafe {
            (*self.data).weak.set((*self.data).weak.get() - 1);
            if (*self.data).weak.get() == 0 && (*self.data).strong.get() == 0 {
                Box::from_raw(self.data);
            }
        }
    }
}

impl<T> Clone for MyWeak<T> {
    fn clone(&self) -> Self {
        unsafe {
            (*self.data).weak.set((*self.data).weak.get() + 1);
        }
        MyWeak{ data: self.data }
    }
}


// Broken examples for Part C
use std::ptr;

pub struct MyBrokenRc<T> {
    data: *mut MyBox<T>,
}

pub struct MyBrokenWeak<T> {
    data: *mut MyBox<T>,
}

impl<T> MyBrokenRc<T> {
    pub fn new(t: T) -> Self {
        MyBrokenRc{ data: Box::into_raw(Box::new(
            MyBox{ data: Box::into_raw(Box::new(t)),
                   strong: Cell::new(1),
                   weak: Cell::new(0),
            }))
        }
    }

    pub fn consume(self) -> Result<T, MyBrokenRc<T>> {
        unsafe {
            if (*self.data).strong.get() == 1 {
                (*self.data).strong.set(0);
                Ok(*Box::from_raw((*self.data).data))
            } else {
                Err(self)
            }
        }
    }

    pub fn downgrade(&self) -> MyBrokenWeak<T> {
        unsafe {
            (*self.data).weak.set((*self.data).weak.get() + 1);
        }
        MyBrokenWeak{ data: self.data }
    }
}

impl<T> Drop for MyBrokenRc<T> {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw((*self.data).data);
            Box::from_raw(self.data);
        } 
    }
}

impl<T> ops::Deref for MyBrokenRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        let result: *const Self::Target = ptr::null();
        unsafe { &*result }
    }
}

impl<T> Clone for MyBrokenRc<T> {
    fn clone(&self) -> Self {
        unsafe {
            (*self.data).strong.set((*self.data).strong.get() + 1);
            MyBrokenRc{ data: self.data }
        }
    }
}

impl<T> MyBrokenWeak<T> {
    pub fn upgrade(&self) -> Option<MyBrokenRc<T>> {
        unsafe {
            if (*self.data).strong.get() > 0 {
                (*self.data).strong.set((*self.data).strong.get() + 1);
                Some(MyBrokenRc{ data: self.data })
            } else {
                None
            }
        }
    }
}


impl<T> Drop for MyBrokenWeak<T> {
    fn drop(&mut self) {
        unsafe {
            (*self.data).weak.set((*self.data).weak.get() - 1);
            if (*self.data).weak.get() == 0 && (*self.data).strong.get() == 0 {
                Box::from_raw(self.data);
            }
        }
    }
}

impl<T> Clone for MyBrokenWeak<T> {
    fn clone(&self) -> Self {
        unsafe {
            (*self.data).weak.set((*self.data).weak.get() + 1);
        }
        MyBrokenWeak{ data: self.data }
    }
}

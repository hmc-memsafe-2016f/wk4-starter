pub struct MyRc<T> {
    d: *mut RcData<T>,
}

pub struct MyWeak<T> {
    d: *mut RcData<T>,
}

// If strongcount > 0, data will be valid. Once strongcount falls to 0, data is
//  deleted
// Once strongcount == 0 and weakcount == 0, this data structure will be deleted
struct RcData<T> {
    data: *mut T,
    strongcount: usize,
    weakcount: usize,
}

impl<T> MyRc<T> {
    pub fn new(t: T) -> MyRc<T> {
        MyRc{d: Box::into_raw(Box::new(
                    RcData{data: Box::into_raw(Box::new(t)),
                           strongcount: 1, weakcount: 0}))}
    }

    pub fn consume(self) -> Result<T, MyRc<T>> {
        unsafe {
            if (*self.d).strongcount == 1 {
                (*self.d).strongcount = 0;
                // This moves data out (and frees the heap-allocated memory).
                // RcData is not deleted, but since self is dropped at the end
                // of this function, the Drop will delete RcData if weakcount is
                // 0.
                Ok(*Box::from_raw((*self.d).data))
            } else {
                Err(self)
            }
        }
    }

    pub fn downgrade(&self) -> MyWeak<T> {
        unsafe {
            (*self.d).weakcount += 1;
        }
        MyWeak{d: self.d}
    }

}

impl<T> std::ops::Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(*self.d).data
        }
    }
}
impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        unsafe{
            (*self.d).strongcount += 1;
        }
        MyRc{d: self.d}
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            // If this was the last strong reference, delete the actual data
            if (*self.d).strongcount == 1 {
                Box::from_raw((*self.d).data);
            }

            // We have one fewer strong count
            // This could be 0 if we were just consumed, and subtracting again
            // would lead to underflow
            if (*self.d).strongcount > 0 {
                (*self.d).strongcount -= 1;
            }
            if (*self.d).strongcount == 0 && (*self.d).weakcount == 0 {
                Box::from_raw(self.d);
            }
        }
    }
}

impl<T> MyWeak<T> {
    pub fn upgrade(&self) -> Option<MyRc<T>> {
        unsafe {
            if (*self.d).strongcount > 0 {
                (*self.d).strongcount += 1;
                Some(MyRc{d: self.d})
            } else {
                None
            }
        }
    }
}


impl<T> Drop for MyWeak<T> {
    fn drop(&mut self) {
        unsafe {
            (*self.d).weakcount -= 1;
            if (*self.d).weakcount == 0 && (*self.d).strongcount == 0 {
                Box::from_raw(self.d);
            }
        }
    }
}

impl<T> Clone for MyWeak<T> {
    fn clone(&self) -> Self {
        unsafe{
            (*self.d).weakcount += 1;
        }
        MyWeak{d: self.d}
    }
}

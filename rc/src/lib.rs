pub struct MyRc<T> {
    d: *mut RcData<T>,
}

struct RcData<T> {
    data: T,
    count: usize,
}

impl<T> MyRc<T> {
    pub fn new(t: T) -> MyRc<T> {
        MyRc{d: Box::into_raw(Box::new(RcData{data: t, count: 1}))}
    }

    pub fn consume(self) -> Result<T, MyRc<T>> {
        unsafe {
            if (*self.d).count == 1 {
                (*self.d).count = 2;
                Ok(Box::from_raw(self.d).data)
            } else {
                Err(self)
            }
        }
    }
}

impl<T> std::ops::Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &(*self.d).data
        }
    }
}
impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        unsafe{
            (*self.d).count += 1;
        }
        MyRc{d: self.d}
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone();
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            let old = (*self.d).count;
            (*self.d).count = old-1;
            if old == 1 {
                Box::from_raw(self.d);
            }
        }
    }
}

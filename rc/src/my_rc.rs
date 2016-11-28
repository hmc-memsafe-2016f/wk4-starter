use MyRefCounter;
use std::ops::Deref;
use std;
use std::mem;

pub struct MyRc<T> {
    data: *mut T,
    refs: *mut usize
}

impl<T> MyRefCounter<T> for MyRc<T> {
    fn new(t: T) -> MyRc<T>
    {
        MyRc{data:Box::into_raw(Box::new(t)), refs : Box::into_raw(Box::new(1))}
    }
    fn consume(self) -> Result<T,MyRc<T>>
    {   
        unsafe
        {
            if *self.refs == 1 {
                let ret_val = std::ptr::read(self.data);

                // manually destruct what is remaining of self to
                // so as to not run the destructor for data. Think this may leak
                // memory since the data is never really cleared...
                Box::from_raw(self.refs);
                mem::forget(self);

                Ok(ret_val)
            } else {
                Err(self)
            }
        }
    }
}

impl<T> Deref for MyRc<T> { // refer to the underlying data
   type Target = T;
   fn deref(&self) -> &Self::Target
   {
        unsafe{&*self.data}
   }
}

impl<T> Clone for MyRc<T> { // Make a new `MyRc` to the same underlying data
    fn clone(&self) -> Self
    {
        unsafe{*self.refs += 1}
        MyRc{data: self.data, refs: self.refs}
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe{
            if *self.refs == 1 {
                Box::from_raw(self.refs);
                Box::from_raw(self.data);
            } else {
                *self.refs -= 1;
            }
        }
    }
}
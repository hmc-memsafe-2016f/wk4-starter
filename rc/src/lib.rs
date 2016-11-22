use std::ops;
use std::mem;
use std::ptr;
use std::marker::PhantomData;
use std::cell::Cell;

struct MyRcBox<T> {
    count: Cell<usize>,
    pointer: *mut T,
}

pub struct MyRc<T> {
    pointer: MyRcBox<T>,
    phantom: PhantomData<T>,
}

impl<T> MyRc<T> {
    fn new(t: T) -> Self {
        MyRc {
            pointer: MyRcBox {
                count: Cell::new(1),
                pointer: unsafe { Box::into_raw(Box::new(t)) }
            },
            phantom: PhantomData
        }
    }

    fn consume(self) -> Result<T, MyRc<T>> {
        if self.pointer.count.get() == 1 {
            Ok(unsafe { ptr::read(self.pointer.pointer) })
        } else {
            Err(self)
        }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        if self.pointer.count.get() < 2 {
            unsafe {
                Box::from_raw(self.pointer.pointer);
            };
        } else {
            self.pointer.count.set(self.pointer.count.get() - 1);
        }
    }
}

impl<T> ops::Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.pointer.pointer }
    }
}

impl<T> ops::DerefMut for MyRc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.pointer.pointer }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        self.pointer.count.set(self.pointer.count.get() + 1);
        MyRc{ pointer: self.pointer, phantom: PhantomData }
    }
}

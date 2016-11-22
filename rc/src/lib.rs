
use std::ops::Deref;

struct MyRC<T> {
  data: Box<T>
}

impl<T> MyRC<T> {
  fn new(t: T) -> MyRC<T> {
      MyRC{data: Box::new(t)}
  }

  fn consume(self) -> Result<T, MyRC<T> > {
      Ok(*self.data)
  }
}

impl<T> Deref for MyRC<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.data
    }
}
impl<T> Clone for MyRC<T> {
    fn clone(&self) -> MyRC<T> {
        MyRC{data: self.data}
    }
}


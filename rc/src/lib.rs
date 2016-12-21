use std::ops::Deref;
use std::mem;

pub struct MyRc<T> {
    value: *mut T,
    pub r_count: *mut usize,
}

impl<T> MyRc<T>{
  pub fn consume(self) -> Result<T,MyRc<T>>{
  	if unsafe{*self.r_count} == 1{
  	   let res = Ok(unsafe{*Box::from_raw(self.value)});
       unsafe{Box::from_raw(self.r_count)}; //destruct the r_count
       mem::forget(self); //but not the value
       res
  	} else {
  		Err(self)
  	}
  }

  pub fn new(t: T) -> MyRc<T>{
    let r_count = Box::into_raw(Box::new(1)); //create raw pointer to the data
    let value = Box::into_raw(Box::new(t));
    MyRc{
      value : value,
      r_count: r_count,
    }
  }
}

impl<T> Deref for MyRc<T> { // refer to the underlying data
   type Target = T;
   
   fn deref(&self) -> &T {
        unsafe{&*self.value}
   }
}

impl<T> Clone for MyRc<T> { // Make a new `MyRc` to the same underlying data
   fn clone(&self) -> MyRc<T>{
   		unsafe{*(self.r_count) += 1;}
   		MyRc{
   			value: self.value,
   			r_count: self.r_count,
   		}
   }
}

impl<T> Drop for MyRc<T>{
	 fn drop(&mut self) {
      unsafe{
        if *self.r_count == 1{
         //destruct by taking ownership
        Box::from_raw(self.value);
        Box::from_raw(self.r_count);
        }
        else {
        *(self.r_count) -= 1;
        }
      } 
   }
}
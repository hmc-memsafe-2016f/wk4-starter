//MyRc

use std::ops::Deref;
use std::mem;

pub struct MyRc<T>
{
	ptr: *mut Counter<T>,
}

pub struct Counter<T>
{
	data: T,
	count: usize,
}

impl<T> MyRc<T>
{
	pub fn new(t: T) -> MyRc<T>
	{
		let c = Box::new(Counter{data: t, count: 1});
		MyRc{ptr: Box::into_raw(c)}
	}

	pub fn consume(self) -> Result<T, MyRc<T>>
	{
		unsafe
		{
			if (*self.ptr).count == 1
			{
				//we're about to drop our only pointer to the data
				//so it's ok to zero it out
				let fake: Counter<T> = mem::uninitialized();

				//get the real value by replacing it with fake
				let real = std::ptr::replace(self.ptr, fake); 

				//doesn't drop real.data
				Ok(real.data)
			}
			else
			{
				Err(self)
			}
		}
	}
}

impl<T> Deref for MyRc<T>
{
	type Target = T;

	fn deref(&self) -> &T
	{
		unsafe
		{
			return &(*self.ptr).data
		}
	}
}

impl<T> Clone for MyRc<T>
{
	fn clone(&self) -> Self
	{
		unsafe
		{
			//up the reference count
			(*self.ptr).count += 1;
		}
		
		//the cloned counter has the same pointer
		MyRc{ptr: self.ptr}
	}
}

impl<T> Drop for MyRc<T>
{
	fn drop(&mut self)
	{
		unsafe
		{
			//if this is the last rc that points to the data...
			if (*self.ptr).count == 1
			{
				//create a box from the pointer...
				let b = Box::from_raw(self.ptr);
				//then let it go out of scope to destruct the data
			}
			else
			{
				(*self.ptr).count -= 1
			}
		}
	}
}

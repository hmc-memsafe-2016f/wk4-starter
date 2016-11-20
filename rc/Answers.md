1. In the `consume` function, if we read pointer `self.core` with `ptr::read`
instead of getting a reference the thing it points to, we will end up moving
out of the `Option<T>` that we coppied instead of the one on the heap. This
will cause the `T` to be `drop()`ed twice: once in the caller to consume, and
once in `MyRc::drop()`, i.e. we created a double delete.

I know this works because I originally had this bug in my code before I was
storing the value in an `Option`.


2. Failing to increment the reference count in `clone()` could cause memory
errors as the underlying core object could be deallocated while there are
still references to it.
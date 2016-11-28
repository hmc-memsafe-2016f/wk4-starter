1. In the `consume` function, if we read the pointer `self.core` with `ptr::read`
instead of getting a reference to the thing it points to, we will end up moving
out of the `Option<T>` that we coppied onto the stack instead of out of the one on the heap. This
will cause the `T` to be `drop()`ed twice: once in the caller to consume, and
once in `MyRc::drop()`. We created a double delete.
I know this works because I originally had this bug in my code before I was
storing the value in an `Option`.

2. Failing to increment the reference count in `clone()` could cause memory
errors as the underlying core object could be deallocated while there are
still references to it.
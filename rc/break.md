1. To break MyRc by chaning something in an unsafe block, we could write the following in `consume`:
```
if *self.count == 1 {
    return Ok(ptr::read(self.ptr));
}
```
This compiles, but will result in double frees if `T` has heap data. Since `ptr::read` makes a copy
of the underlying data, heap data owned by the referent will be freed once when we drop the copy and
once when we drop the original.

2. This would be pretty dumb, but we could fail to count references properly when we call `clone` or
`new`. For instance, if we initialized a cloned MyRc with a lower `count` than its parent, we could
free the parent and have access to freed memory via the clone.

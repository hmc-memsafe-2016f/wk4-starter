Julien Chien

Part C:
1. Describe a way to break MyRc by changing something inside an unsafe block or function.

In `consume`, if we took away the `if` case that checked that the count is 1, then the
value would be deleted/replaced. Since other RCs might have been pointing to it, this might
cause a memory error: another RC might be still pointing to this recently deleted object, which
means that if it is dereferenced, there would be a memory error (the underlying data is gone).

2. Describe a way to break MyRc by changing something outside an unsafe block of function.

If the `new` is changed so that MyRc is not initialized with a count of 1 (say, with a count of
2 or 0 instead), it can potentially cause memory problems, as too many or not enough drops have happened.


# Breaking `MyRc`
## In `unsafe`

An easy way to break `MyRc` in an `unsafe` block is to simply return a null pointer in `deref` instead of the actual data. This will cause a memory error in all programs that use this method in `MyRc` (which should be all of them).

## Outside `unsafe`

Outside of `unsafe`, we could still break `MyRc` by removing the count check from the `drop` function. Then, if a copy of the `MyRc` is made and dropped, the original one will be invalid, too.

# Reference Count Overflow
## Problems With Overflow
If there's a _lot_ of clones of the same `MyRc`, then it's possible that the reference count overflows. If `usize` is 64-bits, then if we get 2<sup>63</sup>+1 clones, then the reference count will wrap around to 1, so if one were dropped, the data would be dropped even though there are a ton of references to it. (In reality, rust will panic when the wrap happens.)

## Overflow Even With `usize`
There cannot possible be `usize` different references to the same variable at once, size the maximum value of `usize` is at least the number of bytes of memory in the system. If `drop`s aren't run, however, it's posible to create a loop that creates and does not drop a ton of `MyRc`s until there are enough for the overflow to happen.

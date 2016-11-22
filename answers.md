## Part C: Breaking Rc

1. We could break `MyRc` by returning an invalid pointer (i.e. a null pointer) inside of `deref` instead of the real data.
2. We could break `MyRc` by not checking the count in `drop`; then if we make a copy and drop that, the original will be as well.

Both of these can be seen from almost any normalusage of `MyRc`; we'll almost always want to dereference it, and `drop` is usually desirable, even if it isn't strictly required to be run by Rust. The compiling implementation can be found in `lib.rs` as `MyBrokenRc` and `MyBrokenWeak`; tests that fail can be found in `requiredBroken.rs` and `weakBroken.rs`. 

## Bonus: Unsafe `replace_with`
The problem is that if a `!panic` occurs, `replace_with` will exit without cleaning things up properly. This results in a situation with unstable values. This can be seen in `replace_with/tests/required.rs`, in the test `safe_memory_error`. Given that we can't recover from an arbitrary `!panic`, and that for something to be memory safe it has to be safe through a `!panic`, it is impossible to implement `replace_with` safely.

## Bonus: Reference Count Overflow

### Problems with overflow
If we had many clones (i.e. 2<sup>sizeof(`usize`)-1</sup>+1), and then we make one more, we'll get wraparound in our count and it'll think there is only 1 copy. Then if any of them are dropped, all of them would be even though there are many references.

### Overflow with `usize`
We can't actually have `usize` references to the same variable, because `usize` is at least the number of bytes of memory in the system, and we wouldn't have every single byte of memory as a reference. Given that we don't guarantee that `drop` is run, however, if we were to recurse or loop indefinitely, and if none of those ever call `drop`, then we could eventually overflow.

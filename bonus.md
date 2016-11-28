Reference Count Overflow Bonus:

1) Overflow would be problematic because if there were a very high number of references
(the max number of usize)  and then we added another reference to it this would overflow
the reference counter to one. Then if one of the Rc's got dropped it would drop the
data because it would think that was the only Rc pointing to the data, when in reality
there were a whole lot of Rcs pointing to the data.

2) If we had a very small address space, such as just at an 8bit or 16bit machine,
then we could create Rcs in a loop and forget about the Rcs (so as to not run the destructor).
Since they are forgotten about we could keep creating Rcs without lowering the counter
in the Rcs thus eventually overflowing the Rc counter.
a) A way to break MyRc by changing something in the unsafe portion of the code is
to add this to the consume function:
```
*self.data = mem::zeroed();
Box::from_raw(self.data);
```
This will compile but will fail when one tries to run the tests by saying: 
error: Process didn't exit successfully

I wanted to do this because it feels like my current implementation leaks the self.data
field since we forget self and we do nothing to free the box the self.data allocated.
It seems to fail the tests somewhat nondeterministically, passing various numbers
of tests each run.

b)
We could change the new function to:
`MyRc{data:Box::into_raw(Box::new(t)), refs : Box::into_raw(Box::new(0))}`
which then produces a memory error because when it checks if `*self.refs == 1`
in the unsafe code it will proceed as if there is only one reference to the data
when in fact there are two, so it will proceed to drop things since it thinks it
is the last remaining rc but then the other rc will try to use the dropped data
and error. The use case that will cause this is creating two MyRc's, most likely
by creating one then cloning it.
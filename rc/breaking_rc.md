# Breaking RC with safe code
In MyRc::new(), if .count is initialized to 0 instead of 1, then if there is one
clone (i.e. two instances of MyRC pointing to the same data) and one is dropped,
drop() will release the memory even though it can still be accessed by 
dereferencing the clone.

# Breaking RC with unsafe code
In MyRc::consume(), if the test condition is changed (say, to .count <= 2), 
then we will replace the RC data to uninitialized memory even though it can
still by accessed through clones.
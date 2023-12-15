This is an attempt to try porting the entire Java implementation into Rust before starting refactoring.
I have been burned by premature refactoring leading to difficulty diagnosing bugs:
- I didn't know how changing an algorithm would affect the performance, because I don't have all the test cases in place yet
- I had to juggle between the different idiosyncracies of the different implementations

, so I want to get the Rust implementation working and compilant to the unit tests first.

I will mostly limit the refactoring to the following: 
- Removing null objects/null with Option
- Rearrangement/pruning of member variables
- Freedom to alter lookup table implementation
- Wrapping existing types with newtypes for expressiveness

The following will be preserved:
- Duplication of different files that are essentially the same
- All Interface APIs

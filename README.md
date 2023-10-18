# What is this ?
This library aims to be a collection of iterators for improved performance or convenience. At the moment there is only a single family of types, those being `StepRangeU*` for all the standard `u*` types.

# StepRangeU* Examples
```rust
// The odd positive integers from 1 to 1000 the traditional way.
let _ = (1..1000).step_by(2);

// The odd positive integers from 1 to 1000 with a constant step size.
let _ = StepRangeU64::<2>::new(1, 1000);

// If you really want to you can also do this... but don't.
let _ = StepRangeU64::<1>::new(1, 1000).step_by(2);
```
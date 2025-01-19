This repository demonstrates two common panic scenarios when working with vectors in Rust:

1. **`pop()` on an empty vector:** Calling the `pop()` method on an empty `Vec` results in a panic because there's no element to remove.
2. **Index out of bounds:** Attempting to access a vector element using an index that is outside the valid range (0 to vector.len() - 1) causes a panic.

The `bug.rs` file contains the code that reproduces these panics. The `bugSolution.rs` file shows how to avoid these panics using safe programming practices.
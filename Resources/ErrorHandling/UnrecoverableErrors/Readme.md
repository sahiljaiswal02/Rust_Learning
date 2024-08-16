# Unrecoverable Errors with panic!
> By default, when a panic occurs the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters.

> Rust, therefore, allows you to choose the alternative of immediately aborting, which ends the program without cleaning up the data.

> Add `panic = 'abort'` to the appropriate [profile] sections in your `Cargo.toml` file.

```toml
[profile.release]
panic = 'abort'
```
## Example:
```rust
fn main() {
    panic!("crash and burn");  //panic will end the program
}
```
## Example:
```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];  // Attempting to access an element beyond the end of a vector, which will cause a call to panic!
}
```
> A backtrace is a list of all the functions that have been called to get to this point. 
```bash
RUST_BACKTRACE = 1 cargo run   #  any value except 0

# Managing Growing Projects with Packages, Crates, and Modules
## Packages: 
> A Cargo feature that lets you build, test, and share crates
- Multiple binary crates
- An optional library crate

### Example
> To create a new package:

```sh
cargo new my_project --bin
```


## Crates: 
> A tree of modules that produces a library or executable
- Binary Crates: Produce an executable file.
- Library Crates: Produce a library that can be used as a dependency.

### Example
> Binary Crate
```rust
fn main() {
    println!("Hello, world!");
}
```
> Library Crate
```rust
pub fn greet() -> String {
    "Hello from the library!".to_string()
}
```
## Modules and use: 
> Let you control the organization, scope, and privacy of paths

### Example
> Create a module
```rust
pub mod utils {
    pub fn print_message() {
        println!("Message from the utils module!");
    }
}

```
> Use a module
```rust
use my_project::utils;

fn main() {
    utils::print_message();
}
```
## Paths: 
> A way of naming an item, such as a struct, function, or module
### Example
> Absolute path
```rust
crate::utils::print_message();
```
> Relative path
```rust
self::utils::print_message();
```

## Encapsulation and Privacy: 
> You can control the visibility of items using pub to expose items and manage encapsulation
### Example
> Define a public function in a module: 
```rust
pub mod my_module {
    pub fn public_function() {
        println!("I am public!");
    }
}
```

## Scopes: 
> Scope refers to the context in which names are defined and resolved. Managing scopes helps avoid name conflicts and makes code easier to read.
### Example
> Function Scope 
```rust
fn my_function() {
    let x = 42;
    println!("{}", x);
}
```
> Avoiding name conflicts
```rust
mod outer {
    pub fn function() {}
}

mod inner {
    pub fn function() {}
}

fn main() {
    outer::function();
    inner::function();
}
```

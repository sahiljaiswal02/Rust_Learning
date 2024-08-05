#### What is Cargo?

Cargo is a package manager that is equivalent to npm or pub, and comes built in with Rust.

#### Get started

1. Create a new project:

```sh
cargo new project_name # Scaffold a new project
cd project_name # Open the project folder
code . # Open VS Code (or text editor of choice)
```

### Project anatomy

#### File Structure

```
project_name
├── Cargo.toml
├── .gitignore
└── src
    └── main.rs
```

#### Cargo.toml

> Equivalent to pubspec.yaml in Dart/Flutter or package.json in JS/TS

```toml
[package]
name = "project_name"
version = "0.1.0"
edition = "2018"

[dependencies]
# Dependencies (crates) go here
```

#### main.rs

> Equivalent to index.js, main.dart, etc.

```rust
// Main application
fn main() {
    // Print to console
    println!("Hello, world!");
}
```

### Build and run a Cargo project

Builds update the file structure accordingly:

```
project_name
...
├── Cargo.lock
└── target
    ├── debug
    └── release
```

#### Debug build

```sh
cargo build
```

#### Release build

```sh
cargo build --release
```

#### Run it

> Builds with optimizations for client delivery

```sh
cargo run

  Compiling rust_in_100 v0.1.0 (/Users/klutch/Desktop/FireshipRust/rust_in_100)
      Finished dev [unoptimized + debuginfo] target(s) in 4.52s
       Running `target/debug/rust_in_100`
Hello, world!
```

#### Check

```sh
cargo check # Verify that it compiles without building
```

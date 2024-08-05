# Learning Rust

Welcome to the Learning Rust repository! Rust is a modern systems programming language that is both fast and memory-efficient. With no runtime or garbage collector, Rust is well-suited for performance-critical services, embedded devices, and easy integration with other languages.

## What is Rust?

Rust offers a range of features that make it an exceptional choice for various programming needs:

- **Blazing Fast and Memory Efficient:** Rust's performance is top-notch, and it does not rely on a runtime or garbage collector.
- **Memory and Thread Safety:** The rich type system and ownership model ensure memory safety and thread safety, eliminating many types of bugs at compile time.
- **Excellent Documentation and Tooling:** Rust comes with comprehensive documentation, a friendly compiler with helpful error messages, and a suite of top-tier tools including:
  - An integrated package manager and build tool (Cargo)
  - Smart multi-editor support with auto-completion and type inspections
  - An auto-formatter and more

## Documentation

- **Official Rust Site:** [rust-lang.org](https://www.rust-lang.org/learn)
- **GitHub Practice Repository:** [rust-lang/rustlings](https://github.com/rust-lang/rustlings/)
- **Examples and Learning Resources:** [rust-lang/rustfmt](https://doc.rust-lang.org/rust-by-example/)

## Installation

To install Rust, use [rustup](https://rust-lang.github.io/rustup/), which will set up Rust and Cargo (the package manager) on your system.

## Creating a New Project with Cargo

Cargo is Rust's package manager and build tool, similar to npm (JavaScript) or pub (Dart). It comes bundled with Rust and is used to manage dependencies, build projects, and more.

To create a new Rust project, use the following command:

```bash
cargo new project_name

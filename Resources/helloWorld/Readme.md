# Getting Started with Rust

Welcome to your first Rust project! This guide will walk you through initializing a new Rust project and running a basic program.

## Project Initialization

To start with Rust, you'll need to create a new Rust file and write a simple program. Here’s how you can do it:

1. **Create a New Rust File**

   Create a file named `main.rs`. This file will contain your Rust code.

2. **Write Your First Program**

   Open `main.rs` and add the following code:

   ```rust
   fn main() {
       println!("Hello, World!");
   }
   
# Compiling and Running Your Rust Program

Once you've written your Rust program, you'll need to compile and run it. Follow these steps to do so:

## Compiling the Program

1. **Open Your Terminal or Command Prompt**

   Navigate to the directory containing your `main.rs` file.

2. **Compile the Program**

   Use the Rust compiler command to compile your Rust code:

   ```bash
   rustc main.rs

This command will generate several files:

- main.rs (your source file)
- main.exe (the executable file)
- A debugging information file (with a .pdb extension, on Windows)


After compilation, run the generated executable:
   ```bash
   .\main.exe
   ```
You should see the output:
```rust
Hello, World!

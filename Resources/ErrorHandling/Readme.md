# Error Handling in Rust

Rust provides robust error handling mechanisms to ensure your programs are resilient and reliable. Here's a summary of the key concepts covered in this chapter:

1. **Categories of Errors**:
   - **Recoverable Errors**: These are errors you can handle and recover from, such as file not found or invalid input.
   - **Unrecoverable Errors**: These indicate bugs or critical issues, like array index out of bounds, and typically require immediate termination of the program.

2. **No Exceptions**: Unlike many other languages, Rust does not use exceptions for error handling. Instead, it provides distinct mechanisms for managing different types of errors.

3. **`Result<T, E>` Type**:
   - Used for recoverable errors.
   - `Result` is an enum that can be either `Ok(T)` for successful outcomes or `Err(E)` for errors.
   - Encourages handling errors explicitly and ensures you address possible failure points.

4. **`panic!` Macro**:
   - Used for unrecoverable errors.
   - `panic!` immediately stops execution and unwinds the stack, typically used for bugs or critical issues.

5. **Error Handling Best Practices**:
   - Use `Result<T, E>` to handle errors gracefully and provide feedback to users or retry operations.
   - Use `panic!` for situations where continuing execution is not possible or safe.

6. **Explicit Error Handling**:
   - Rust’s design requires you to acknowledge and handle errors explicitly, making it less likely to overlook them.

7. **Error Propagation**:
   - You can propagate errors using the `?` operator, which simplifies error handling by passing errors up the call stack.

8. **Custom Error Types**:
   - Define your own error types to provide more context and information about failures specific to your application.

9. **Handling Errors Appropriately**:
   - Decide whether to recover from an error or stop execution based on the nature of the error and the context in which it occurs.

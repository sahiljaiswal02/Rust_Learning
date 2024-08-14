# Recoverable Errors with Result
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
> `T` represents the type of the value that will be returned in a success case within the Ok variant

> `E` represents the type of the error that will be returned in a failure case within the Err variant

#### Example:
> Using a `match` expression to handle the Result variants that might be returned.
```rust
use std::fs::File;  //file library

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {  
        Ok(file) => file,  // If the file is OK it will return the inner file value
        Err(error) => panic!("Problem opening the file: {error:?}"), // If not then return error 
    };
}
```
## Matching on Different Errors
> Nested Match
```rust
fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {  // This struct has a method kind that we can call to get an io::ErrorKind value. 
            ErrorKind::NotFound => match File::create("hello.txt") {  //ErrorKind::NotFound, which indicates the file we’re trying to open doesn’t exist yet.
                Ok(fc) => fc,  // If file creation is successful. 
                Err(e) => panic!("Problem creating the file: {e:?}"),  // If there is a fail in creation the file.
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");  // If any other error occurs.
            }
        },
    };
}
```
## Shortcuts for Panic on Error: `unwrap` and `expect`
> If the Result value is the Ok, `unwrap` will return the value inside the Ok. If the Result is the Err variant, `unwrap` will call the panic! 
```rust
fn main() {
    let greeting_file = File::open("hello.txt").unwrap();  
}
```

> `expect` method lets us also choose the panic! error message.
```rust
fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project"); 
} 
```
## Propagating Errors
> When a function’s implementation calls something that might fail, instead of handling the error within the function itself you can return the error to the calling code so that it can decide what to do. This is known as propagating the error and gives more control to the calling code.

```rust
use std::io::{self, Read};
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),  //If the file doesn’t exist or can’t be read, this function will return those errors to the code that called the function.
    };
    let mut username = String::new();
}
```

## A Shortcut for Propagating Errors: the `?` Operator
> `?` converts the error types without needing to add any more code to the function.
```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?; // This skips all the rest of the boilerplate code and`?` does all the work if the code fails.
    Ok(username)
}
```

## Where The `?` Operator Can Be Used
> The `?` operator can only be used in functions whose return type is compatible with the value the `?` is used.

#### Example:
```rust
fn main() {
    let greeting_file = File::open("hello.txt")?;  // This is returning file not result so this will not complile and throw error
}
```
- We’re only allowed to use the `?` operator in a function that returns Result, Option, or another type that implements FromResidual.
- To fix that error either use a compatible return type with `?` or use match.

#### Solution:
```rust
fn main() -> Result<(), Box<dyn Error>> {  //Changing main to return Result<(), E> allows the use of the ? operator on Result values.
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
```

#### Example: The ` last_char_of_first_line ` function

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

- This code takes the text string slice argument and calls the `lines` method on it, which returns an iterator over the lines in the string. 
- Because this function wants to examine the first line, it calls `next` on the iterator to get the first value from the iterator.
- If text is the empty string, this call to `next` will return None, in which case we use `?` to stop and return None from `last_char_of_first_line`.
- If text is not the empty string, `next` will return a Some value containing a string slice of the first line in text.
- We can call `chars` on that string slice to get an iterator of its characters. We’re interested in the last character in this first line, so we call `last` to return the last item in the iterator.

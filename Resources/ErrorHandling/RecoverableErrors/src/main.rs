// Recoverable Errors with Result
enum Result<T, E> {
    Ok(T),
    Err(E),
}

//T represents the type of the value that will be returned in a success case within the Ok variant
// E represents the type of the error that will be returned in a failure case within the Err variant

use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {  //Using a match expression to handle the Result variants that might be returned
        Ok(file) => file,  // If the file is OK it will return the inner file value
        Err(error) => panic!("Problem opening the file: {error:?}"), // If not then return error 
    };
}

// Matching on Different Errors
use std::fs::File;
use std::io::ErrorKind;

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

// Shortcuts for Panic on Error: unwrap and expect
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();  //If the Result value is the Ok, unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! 
}

use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");  // expect method lets us also choose the panic! error message.
} 

// Propagating Errors
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),  //If the file doesn’t exist or can’t be read, this function will return those errors to the code that called the function.
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),  // If the proper file can’t be read, this function will return those errors to the code that called the function.
    }
}

// A Shortcut for Propagating Errors: the ? Operator
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;  // `?` converts the error types without needing to add any more code to the function.
    Ok(username)
}

// Example: 
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;  // This skips all the rest of the boilerplate code and`?` does all the work if the code fails.
    Ok(username)
}

use std::fs;
use std::io;


// read_to_string function
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")  //fs::read_to_string function that opens the file, creates a new String, reads the contents of the file, puts the contents into that String, and returns it. 
}


// Example:
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")?;  // This is returning file not result so this will not complile and throw error
}   

// Example:
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {  //Changing main to return Result<(), E> allows the use of the ? operator on Result values.
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}


// Example: The last_char_of_first_line function
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

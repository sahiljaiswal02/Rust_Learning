fn main() {
    println!("Hello, world!");

    another_function();  //Call a function within a function
}

fn another_function() {
    println!("Another function.");
}

// Parameters
fn main() {
    another_function(5);  //Any value is passed within a function as a parameter for the function to use
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}


// Function to return value
fn five() -> i32 { //function with return type
    5
}

fn main() { 
    let x = five();
    println!("The value of x is: {x}");
}


// Function with return
fn main() { 
    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1  // if we put semicolon at the end of this line the it will convert the line into statement and will throw an error
}
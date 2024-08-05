use rand::Rng; //Library to generate random number
use std::cmp::Ordering;
use std::io; //library to take input //library to compare two values
fn main() {
    println!("Welcome to Guess The Number Game");
    let secret_number = rand::thread_rng().gen_range(1..=100); //rand::thread_rng to generate random number and gen_range to put a range between 1 and 100
                                                               // println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess");

        let mut guess = String::new(); //the number will be stored in a new empty string guess
        io::stdin()
            .read_line(&mut guess) //take input from the user and append that into a string (without overwriting its contents)
            //read_line puts whatever the user enters into the string we pass to it, but it also returns a Result value.
            .expect("Failed to read line"); //if the input works fine it will run ok else print the expect message if given else err message

        let guess: u32 = match guess.trim().parse() {
            //trim() function removes whitespaces from the beginning and end of the string and parse will convert the string to the type given (for here its u32).
            Ok(num) => num, //if the parse is able to convert the string to integer it will go with ok
            Err(_) => continue, //else it will ignore the error and go to the next iteration
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            //cmp() function compares two values and returns an enumeration value
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

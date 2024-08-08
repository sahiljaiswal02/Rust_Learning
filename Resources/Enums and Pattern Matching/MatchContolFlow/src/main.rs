// Rust has an extremely powerful control flow construct called match that allows you to compare a value against a series of patterns and then execute code based on which pattern matches
// Example: coin-sorting machine: coins slide down a track with variously sized holes along it, and each coin falls through the first hole it encounters that it fits into.

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
// with if, the condition needs to evaluate to a Boolean value, but here it can be any type


//For multiple line code we use the curly brackets {} in a match arm code
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Patterns That Bind to Values
// Match arms is that they can bind to the parts of the values that match the pattern
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

// Matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,  // If nothing is passed in, return None
        Some(i) => Some(i + 1), //If something is passed in, return Some(i + 1)
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

//Sometimes it also gets exhaustive because rust understand that every case is not covered for example in the following func none cse is not handled so it throws error 
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {  //None case not covered
        Some(i) => Some(i + 1),
    }
}

// Catch-all Patterns and the _ Placeholder

// Game: If num comes other than 3 and 7 then it moves the number of space forward in the board
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other), //uses the variable by passing it to the move_player function
} 

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

// _ is a special pattern that matches any value and does not bind to that value
// This tells Rust we aren’t going to use the value, so Rust won’t warn us about an unused variable.

// Game: If num comes other than 3 and 7 then re roll the dice in the board
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),  //we’re explicitly ignoring all other values in the last arm
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}

// Game: If num comes other than 3 and 7 then don not pass anything
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),  //we aren’t going to use any other value that doesn’t match a pattern
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}



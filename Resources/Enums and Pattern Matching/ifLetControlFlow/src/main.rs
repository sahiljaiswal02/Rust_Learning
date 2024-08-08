//using match
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {max}"), //Some variant by binding the value to the variable max in the pattern. We don’t want to do anything with the None value
    _ => (),  //add _ => () after processing just one variant,
}

// Using if let 
let config_max = Some(3u8);
if let Some(max) = config_max {  //The code in the if let block isn’t run if the value doesn’t match the pattern.
    println!("The maximum is configured to be {max}");
}


//using match
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    _ => count += 1,
}

// Using if let else 
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {state:?}!");
} else {
    count += 1;
}

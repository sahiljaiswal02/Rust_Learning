# Concise Control Flow with if let
> The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest
#### Using match
```rust
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {max}"), //Some variant by binding the value to the variable max in the pattern. We don’t want to do anything with the None value
    _ => (),  //add _ => () after processing just one variant,
}
```
#### Using if let 
> Using if let means less typing, less indentation, and less boilerplate code
```rust
let config_max = Some(3u8);
if let Some(max) = config_max {  //The code in the if let block isn’t run if the value doesn’t match the pattern.
    println!("The maximum is configured to be {max}");
}
```
- In other words, you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.

#### Using match
```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    _ => count += 1,
}
```
#### Using if let else 
```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {state:?}!");
} else {
    count += 1;
}
```

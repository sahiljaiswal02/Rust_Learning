# Functions
- fn keyword, which allows you to declare new functions.
- Rust code uses snake case as the conventional style for function
 ```rust
  fn main() {
      println!("Hello, world!");
  
      another_function();  // Call a function within a function
  }
  
  fn another_function() {
      println!("Another function.");
  }
 ```

## Parameters
> Function has parameters, you can provide it with concrete values for those parameters
  ```rust
  fn main() {
      another_function(5);  // Any value is passed within a function as a parameter for the function to use
  }
  
  fn another_function(x: i32) {
      println!("The value of x is: {x}");
  }
  ```

## Statements and Expressions
#### Expressions:
> Expressions evaluate to a resultant value and do not include ending semicolons and it can return the value.
  ```rust
  fn plus_one(x: i32) -> i32 {
      x + 1 
  }
  ```
#### Statements:
>  Statements are instructions that perform some action and do not return a value.
  ```rust
  fn main() {
      let y = 6;
  }
  ```


## Functions with Return Values
> Functions can return values to the code that calls them by declaring their type after an arrow (->)
  ```rust
  fn five() -> i32 { // function with return type
      5
  }
  
  fn main() { 
      let x = five();
      println!("The value of x is: {x}");
  }
  ```

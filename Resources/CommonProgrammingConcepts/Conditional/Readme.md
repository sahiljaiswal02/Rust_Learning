# Conditionals
> The ability to run some code depending on whether a condition is true and to run some code repeatedly while a condition is true.
### if Expressions
  ```bash
  if number != 0 {
          println!("number was something other than zero");
      }
  ```
### if-else-if
  ```bash
      let number = 6;
  
      if number % 4 == 0 {
          println!("number is divisible by 4");
      } else if number % 3 == 0 {
          println!("number is divisible by 3");
      } else if number % 2 == 0 {
          println!("number is divisible by 2");
      } else {
          println!("number is not divisible by 4, 3, or 2");
      }
  ```
### Single line condition
  ```bash
      let condition = true;
      let number = if condition { 5 } else { 6 };
  
      println!("The value of number is: {number}");
  ```

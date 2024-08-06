# Loops
> It’s useful to execute a block of code more than once.
## Conditional Loops with For
#### Infinite Loop
  ```bash
  fn main() {
      loop {
          println!("again!");
      }
  }
  ```

#### Returning values from Loop
  ```bash
  fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
  ```
- break expression you use to stop the loop

#### Loop Labels to Disambiguate Between Multiple Loops
   ```bash
   'counting_up: loop {
         println!("count = {count}");
   }
   ```
#### For Loop with range
  ```bash
  fn main() {
      for number in (1..4).rev() { # rev is used to reverse the string
          println!("{number}!");
      }
      println!("LIFTOFF!!!");
  }
  ```

## Conditional Loops with while
  ```bash
  fn main() {
      let mut number = 3;
  
      while number != 0 {
          println!("{number}!");
  
          number -= 1;
      }
  
      println!("LIFTOFF!!!");
  }
  ```
## Loops to Print Array
  ```bash
  fn main() {
      let a = [10, 20, 30, 40, 50];
      let mut index = 0;
  
      while index < 5 {
          println!("the value is: {}", a[index]);
  
          index += 1;
      }
  }
  ```
### Efficient Way because might be that the index provided in while loop is not present 👇
```bash
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```
  

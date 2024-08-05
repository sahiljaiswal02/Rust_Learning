# Programming a Guessing Game
## Setting Up a New Project
  ```bash
  $ cargo new guessing_game
  $ cd guessing_game
  ```
#### Look at the generated Cargo.toml file:
  ```bash
  [package]
  name = "guessing_game"
  version = "0.1.0"
  edition = "2021"
  
  # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
  
  [dependencies]
  ```
#### 1. The first part of the guessing game program will ask for user input, process that input, and check that the input is in the expected form. 
  ```bash
  use std::io;
  
  fn main() {
      println!("Guess the number!");
  
      println!("Please input your guess.");
  
      let mut guess = String::new();
  
      io::stdin()
          .read_line(&mut guess)
          .expect("Failed to read line");
  
      println!("You guessed: {}", guess);
  }
  ```
#### 2. Generating a Secret Number
> Before we can write code that uses rand, we need to modify the Cargo.toml file to include the rand crate as a dependency. 
  ```bash
  [dependencies]
  rand = "0.8.5"
  ```
> After updating the registry, Cargo checks the [dependencies] section and downloads any crates listed that aren’t already downloaded
  ```bash
  $ cargo build
     Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
      Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
  ```
#### 3. Generating a Random Number
  ```bash
      let secret_number = rand::thread_rng().gen_range(1..=100);
  ```
#### 4. Comparing the Guess to the Secret Number
  ```bash
      match guess.cmp(&secret_number) {
          Ordering::Less => println!("Too small!"),
          Ordering::Greater => println!("Too big!"),
          Ordering::Equal => println!("You win!"),
      }
  ```

#### 5. Allowing Multiple Guesses with Looping
  ```bash
      loop {
          // --snip--
      }
  ```

#### 6. Quitting After a Correct Guess
  ```bash
          // --snip--
  
          match guess.cmp(&secret_number) {
              Ordering::Less => println!("Too small!"),
              Ordering::Greater => println!("Too big!"),
              Ordering::Equal => {
                  println!("You win!");
                  break;
              }
          }
  ```

#### 7. Handling Invalid Input
  ```bash
          let guess: u32 = match guess.trim().parse() {
              Ok(num) => num,
              Err(_) => continue,
          };
  ```

### At this point, you’ve successfully built the guessing game. Congratulations!

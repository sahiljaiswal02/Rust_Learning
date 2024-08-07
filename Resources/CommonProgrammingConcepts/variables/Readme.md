# Variables and Mutability
- By default, variables are immutable
- Syntax: Typically declared with a keyword like let and const.

#### Imutable: 
> Once set, their values cannot be changed.
  ```rust
  let x = 5;
  ```
#### Mutable : 
> Their values can be changed.
  ```rust
  let mut x = 5; // mut is used to make a variable mutable
  ```
#### Constants:
> Constants are values that are bound to a name and are not allowed to change. (Imutable)
  ```rust
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
  ```

## Shadowing
> A new variable with the same name as an existing variable is introduced in a scope.
- Effect: The new variable overshadows the old one within that scope.
  ```rust
  fn Shadowing() {
      let x = 5;
      let x = x + 1;
      {
          let x = x * 2;
          println!("The value of x in the inner scope is: {x}"); // When that scope is over, the inner shadowing ends
          // Output: 12
      }
  
      println!("The value of x is: {x}");  // Output: 6
  }
  ```

## Data Types
#### Scaler
> Represents a single value
- Integers: i32, u32 (signed and unsigned).
- Floating-Point: e.g., f32, f64.
- Booleans: bool.
- Characters: char.
#### Compound
> Groups multiple values
- Tuples: Fixed-size, heterogeneous collection of values.
    ```rust
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0; // five_hundred = 500
    let six_point_four = x.1; // six_point_four = 6.4
    let one = x.2; // one = 1
    ```
- Arrays: Fixed-size, homogeneous collection of values
    ```rust
    let a = [1, 2, 3, 4, 5];
    let first = a[0]; // first = 1
    let second = a[1]; // second = 2
    ```



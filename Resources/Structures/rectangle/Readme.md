# An Example Program Using Structs ( Area of Rectangle)
> Finding area of rectangle
### Using basic function
> The area function is supposed to calculate the area of one rectangle, but the function we wrote has two parameters, and it’s not clear anywhere in our program that the parameters are related.
  ```rust
  fn main() {
      let width1 = 30;
      let height1 = 50;
  
      println!(
          "The area of the rectangle is {} square pixels.",
          area(width1, height1)
      );
  }
  fn area(width: u32, height: u32) -> u32 {
      width * height
  }
  ```
Output: 
```bash
The area of the rectangle is 1500 square pixels.
```

### Using Tuple
> We would have to keep in mind that width is the tuple index 0 and height is the tuple index 1. This would be even harder for someone else to figure out and keep in mind if they were to use our code
  ```rust
  fn main() {
      let rect1 = (30, 50);
  
      println!(
          "The area of the rectangle is {} square pixels.",
          area(rect1)
      );
  }
  fn area(dimensions: (u32, u32)) -> u32 {
      dimensions.0 * dimensions.1
  }
  ```
Output: 
```bash
The area of the rectangle is 1500 square pixels.
```

### Using Struct
> This gives the proper clarity that width and height are related to each other, and it gives descriptive names to the values rather than using the tuple index values of 0 and 1.
  ```rust
  struct Rectangle {
      width: u32,
      height: u32,
  }
  fn main() {
      let rect1 = Rectangle {
          width: 30,
          height: 50,
      };
  
      println!(
          "The area of the rectangle is {} square pixels.",
          area(&rect1)  // we want to borrow the struct rather than take ownership of it
      );
  }
  fn area(rectangle: &Rectangle) -> u32 {
      rectangle.width * rectangle.height
  }
  ```
Output: 
```bash
The area of the rectangle is 1500 square pixels.
```

### Adding Useful Functionality with Derived Traits
>  It shows the values of all the fields for this instance, which would definitely help during debugging.
  ```rust
  struct Rectangle {
      width: u32,
      height: u32,
  }
  
  fn main() {
      let rect1 = Rectangle {
          width: 30,
          height: 50,
      };
  
      println!("rect1 is {}", rect1); 
  
  }
  ```
> This will thorugh an error
```rust
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```
> To remove the error we add an attribute
```rust
 #[derive(Debug)]  // we add the outer attribute #[derive(Debug)] just before the struct definition
```
> Now the output will be:
  ```bash
  cargo run
     Compiling rectangles v0.1.0 (file:///projects/rectangles)
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
       Running `target/debug/rectangles`
  rect1 is Rectangle { width: 30, height: 50 }
  ```


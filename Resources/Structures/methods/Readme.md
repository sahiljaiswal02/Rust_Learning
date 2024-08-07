# Method
> Methods are similar to functions
> Their first parameter is always self, which represents the instance of the struct the method is being called on.

> The main reason for using methods instead of functions, in addition to providing method syntax and not having to repeat the type of self in every method’s signature, is for organization. 

### Finding area of the rectangle using Method
  ```rust
  #[derive(Debug)]
  struct Rectangle {
      width: u32,
      height: u32,
  }
  
  impl Rectangle {    //impl block are called associated functions
      fn area(&self) -> u32 {  //we use &self instead of rectangle: &Rectangle, &self here for the same reason we used &Rectangle
          self.width * self.height
      }
  }
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
  ```
### Using method with condition (Another way)
  ```rust
  impl Rectangle {
      fn width(&self) -> bool { //give a method the same name as one of the struct’s fields
          self.width > 0  //If width is greater than 0 then only it will run else throw an error
      }
  }
  
  fn main() {
      let rect1 = Rectangle {
          width: 30,
          height: 50,
      };
  
      if rect1.width() {
          println!("The rectangle has a nonzero width; it is {}", rect1.width);
      }
  }
  ```

### Methods with More Parameters
> we want an instance of Rectangle to take another instance of Rectangle and return true if the second Rectangle can fit completely within self (the first Rectangle); otherwise, it should return false.

  ```rust
  fn main() {
      let rect1 = Rectangle {
          width: 30,
          height: 50,
      };
      let rect2 = Rectangle {
          width: 10,
          height: 40,
      };
      let rect3 = Rectangle {
          width: 60,
          height: 45,
      };
  
      println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
      println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
  }
  
  impl Rectangle {
      fn area(&self) -> u32 {  //To calculate the area
          self.width * self.height
      }
  
      fn can_hold(&self, other: &Rectangle) -> bool {  //To check if rect1 can hold rect2 and return bool
          self.width > other.width && self.height > other.height
      }
  }
  ```
Output:
  ```bash
  Can rect1 hold rect2? true
  Can rect1 hold rect3? false
  ```
### Where’s the -> Operator?
> Rust doesn’t have an equivalent to the -> operator; instead, Rust has a feature called automatic referencing and dereferencing

### Assiociated Functions
> All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl. We can define associated functions that don’t have self as their first parameter

> Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct.
  ```rust
  impl Rectangle {
      fn area(&self) -> u32 {  //To calculate the area
          self.width * self.height
      }
  
      fn square(size: u32) -> Self {
          Self {
              width: size,
              height: size,
          }
      }
  }
  fn main(){
      let sq = Rectangle::square(3);
      println!("The area of square is {}", sq.area());
  }
  ```
### Multiple impl Blocks
> We can separate these methods into multiple impl blocks here, but this is valid syntax.
  ```rust
  impl Rectangle {
      fn area(&self) -> u32 {
          self.width * self.height
      }
  }
  
  impl Rectangle {
      fn can_hold(&self, other: &Rectangle) -> bool {
          self.width > other.width && self.height > other.height
      }
  }
  ```


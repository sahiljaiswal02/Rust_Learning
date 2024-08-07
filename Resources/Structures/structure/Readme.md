# Defining and Instantiating Structs
> Structs are similar to tuples

> Unlike with tuples, in a struct you’ll name each piece of data so it’s clear what the values mean. Adding these names means that structs are more flexible than tuples: you don’t have to rely on the order of the data to specify or access the values of an instance.

## Initialization
> Structs are inintialized by the keyword struct.
```bash
  struct User {
      active: bool,
      username: String,
      email: String,
      sign_in_count: u64,
  }
  ```
> To use a struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields.
  ```bash
  fn main() {
      let user1 = User {
          active: true,
          username: String::from("someusername123"),
          email: String::from("someone@example.com"),
          sign_in_count: 1,
      };
  }
  ```
> To make the instance mutable
  ```bash
  let mut user1 = User {
          active: true,
          username: String::from("someusername123"),
          email: String::from("someone@example.com"),
          sign_in_count: 1,
      };
  ```

## Returning Instance
> build_user function will return the instance User.

> Username and email take the values from the parameters passed.
  ```bash
  fn build_user(email: String, username: String) -> User {
      User {
          active: true,
          username: username,
          email: email,
          sign_in_count: 1,
      }
  }
  ```
> Another way
  ```bash
  fn build_user(email: String, username: String) -> User {
      User {
          active: true,
          username,
          email, # email field and the email parameter have the same name, we only need to write email rather than email: email.
          sign_in_count: 1,
      }
  }
  ```
- Because the parameter names and the struct field names are exactly the same.
- we can use the field init shorthand syntax to rewrite build_user.

## Creating Instances from Other Instances with Struct Update Syntax
> It’s often useful to create a new instance of a struct that includes most of the values from another instance.

> Strings are not copied they are directly moved to other instance.
  ```bash
  struct User {
      active: bool,
      username: String,
      email: String,
      sign_in_count: u64,
  }
  
  fn main() {
      let user1 = User {
          email: String::from("someone@example.com"),
          username: String::from("someusername123"),
          active: true,
          sign_in_count: 1,
      };
  
      let user2 = User {
          active: user1.active, # Using user1's active field
          username: user1.username, # Using user1's username field
          email: String::from("another@example.com"),
          sign_in_count: user1.sign_in_count, # Using user1's sign_in_count field
      };
  }
  ```
> Another way for creating Instance with other Instance
  ```bash
  fn main() {
      # --snip--
      let user2 = User {
          email: String::from("another@example.com"),
          ..user1 # The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
      };
  }
  ```
## Using Tuple Structs Without Named Fields to Create Different Types
> Rust also supports structs that look similar to tuples, called tuple structs.

> Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples.
 ```bash
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);
  
  fn main() {
      let black = Color(0, 0, 0);
      let origin = Point(0, 0, 0);
  }
  ```
## Unit-Like Structs Without Any Fields
> You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to ()

> Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself
  ```bash
  struct AlwaysEqual;
  fn main() {
      let subject = AlwaysEqual;
  }
  ```

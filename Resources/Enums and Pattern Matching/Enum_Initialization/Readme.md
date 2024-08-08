# Defining an Enum
> enums give you a way of saying a value is one of a possible set of values.

#### Example:
> Any IP address can be either a version four or a version six address, but not both at the same time.
That property of IP addresses makes the enum data structure appropriate because an enum value can only be one of its variants.
```rust
enum IpAddrKind {  //Enums are used to make variants of types
    V4,
    V6,
}
```
> You can put any kind of data inside an enum variant: strings, numeric types, structs or another Enum.
#### Example:  A Message enum whose variants each store different amounts and types of values
```rust
enum Message {
    Quit,  //has no data associated with it at all
    Move { x: i32, y: i32 }, //has named fields, like a struct does
    Write(String), //includes a single String
    ChangeColor(i32, i32, i32), //includes three i32 values
}
```

### Using methods with Enum
> we’re able to define methods on structs using impl, we’re also able to define methods on enums. Here’s a method named call that we could define on our Message Enum

```rust
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
let m = Message::Write(String::from("hello"));
m.call();
```

### The Option Enum and Its Advantages Over Null Values
> Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent.
```rust
enum Option<T> {
    None,
    Some(T),
}
```

### `<T>` means that the Some variant of the Option enum can hold one piece of data of any type, and that each concrete type that gets used in place of T makes the overall Option<T> type a different type
```rust
let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;
```
#### Example:
```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;  //Error: Can't add an i8 and an Option<i8>, because they’re different types
```

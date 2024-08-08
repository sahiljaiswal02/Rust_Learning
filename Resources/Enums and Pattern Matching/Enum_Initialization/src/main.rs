enum IpAddrKind {  //Enums are used to make variants of types
    V4,
    V6,
}

//instances of each of the two variants of IpAddrKind
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

//for instance, define a function that takes any IpAddrKind:
fn route(ip_kind: IpAddrKind) {}

// And we can call this function with either variant:
route(IpAddrKind::V4);
route(IpAddrKind::V6);

//using enum with struct
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {  //We’ve used a struct to bundle the kind and address values together,
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};

//representing the same concept using just an enum is more concise: rather than an enum inside a struct,
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));

//Another way
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));


// Example:  A Message enum whose variants each store different amounts and types of values
enum Message {
    Quit,  //has no data associated with it at all
    Move { x: i32, y: i32 }, //has named fields, like a struct does
    Write(String), //includes a single String
    ChangeColor(i32, i32, i32), //includes three i32 values
}

// Using methods with enum
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
let m = Message::Write(String::from("hello"));
m.call();

// The Option Enum and Its Advantages Over Null Values
enum Option<T> {
    None,
    Some(T),
}

// Some: Some variant of the Option enum can hold one piece of data of any type.
let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;

//Example:
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;  //Error: Can't add an i8 and an Option<i8>, because they’re different types






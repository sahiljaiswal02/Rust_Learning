// Creating a New String
let mut s = String::new(); // new, empty String

// Creating a String with a Value
let data = "initial contents";
let s = data.to_string();  //This code creates a string containing initial contents.
let s = "initial contents".to_string();  // the method also works on a literal directly
let s = String::from("initial contents"); // the method also works on a literal directly

// Updating a String
// Appending to a String with push_str and push
let mut s = String::from("foo");
s.push_str("bar");  // s will contain foobar


let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);  // we don’t necessarily want to take ownership of the parameter
println!("s2 is {s2}");  // s2 is still valid here

//Example:
let mut s = String::from("lo");
s.push('l'); //s will contain lol.

// Concatenation with the + Operator or the format! Macro
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
// the type of &s2 is &String

// If we need to concatenate multiple strings, the behavior of the + operator gets unwieldy:
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;  //tic-tac-toe

// For combining strings in more complicated ways, we can instead use the format! macro
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3}");  //tic-tac-toe

// Indexing into Strings
let s1 = String::from("hello");
let h = s1[0];  //Error

// Internal Representation
let hello = "Здравствуйте";
let answer = &hello[0];  //don’t want the byte value returned so not valid

// Slicing Strings
let hello = "Здравствуйте";
// s will be a &str that contains the first four bytes of the string
let s = &hello[0..4];  //s will be Зд. 


// Methods for Iterating Over Strings
// For individual Unicode scalar values, use the chars method
for c in "Зд".chars() {
    println!("{c}");  // 3 д
}

// To return bytes
for b in "Зд".bytes() {
    println!("{b}");  // [208, 151, 208, 180]
}


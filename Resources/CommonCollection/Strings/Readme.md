# Storing UTF-8 Encoded Text with Strings
> Strings are implemented as a collection of bytes, plus some methods to provide useful functionality when those bytes are interpreted as text.

## What Is a String?
> The String type  is a growable, mutable, owned, UTF-8 encoded string type.

## Creating a New String
```rust
let mut s = String::new(); // new, empty String
```

## Creating a String with a Value
```rust
let data = "initial contents";
let s = data.to_string();  //This code creates a string containing initial contents.
let s = "initial contents".to_string();  // the method also works on a literal directly
```
> To fix the initials so that string will be added after the initial string
```rust
let s = String::from("initial contents"); // the method also works on a literal directly
```
## Updating a String
> Appending to a String with push_str and push
```rust
let mut s = String::from("foo");
s.push_str("bar");  // s will contain foobar
```
- ` fn add(self, s: &str) -> String ` , We don't take the ownership of the parameter
```rust
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2); 
println!("s2 is {s2}");  // s2 is still valid here
```

## Concatenation with the `+` Operator or the format! Macro
```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```
- We can only add a `&str` to a `String;` we can’t add two String values together. Thats why we used ref of s2 with s1
- The type of `&s2` is `&String`
> If we need to concatenate multiple strings:
```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = s1 + "-" + &s2 + "-" + &s3;  //tic-tac-toe
```
- But in this way it looks complicated and for long strings it will be more. So to simplify it we use `format!`
  
## For combining strings in more complicated ways, we can instead use the `format!` macro
```rust
let s = format!("{s1}-{s2}-{s3}");  //tic-tac-toe
```

## Indexing into Strings
> Indexing in rust is different from others.
```rust
let s1 = String::from("hello");
let h = s1[0];  //Error
```

## Internal Representation
> This is the way to access the byte code
```rust
let hello = "Здравствуйте";
let answer = &hello[0];  //don’t want the byte value returned so not valid
```

## Bytes and Scalar Values and Grapheme Clusters! Oh My!
> Three types :  bytes, scalar values, and grapheme clusters (the closest thing to what we would call letters)
- Bytes:
  > [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
- Scaler:
  > ['न', 'म', 'स', '्', 'त', 'े']
- Grapheme:
  > ["न", "म", "स्", "ते"]

## Slicing Strings
```rust
let hello = "Здравствуйте";
// s will be a &str that contains the first four bytes of the string
let s = &hello[0..4];  //s will be Зд. 
```
- If we were to try to slice only part of a character’s bytes with something like ` &hello[0..1] `, Rust would panic at runtime 

## Methods for Iterating Over Strings
> For individual Unicode scalar values, use the chars method
```rust
for c in "Зд".chars() {
    println!("{c}");  // 3 д
}
```
> To return bytes
```rust
for b in "Зд".bytes() {
    println!("{b}");  // [208, 151, 208, 180]
}
```




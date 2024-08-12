# Storing Lists of Values with Vectors
> Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.

> Vectors can only store values of the same type.

## Creating a new vector
```rust
let v: Vec<i32> = Vec::new();  
let v = vec![1, 2, 3];
```

## Updating a Vector
```rust
let mut v = Vec::new();  // We use push to add the elements in the vector
v.push(5);  
v.push(6);
v.push(7);
v.push(8);
```

## Reading Elements of Vectors
> There are two ways to reference a value stored in a vector: via indexing or by using the get method
```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];  // Indexing method
println!("The third element is {third}");

let third: Option<&i32> = v.get(2);  //Get method
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}
```

- Indexing method panics when the requested index is out of bounds
- Get method returns an None instead of panicking when the requested index is out of bounds
  
## Iterating Over the Values in a Vector
```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{i}");
}
```

> We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements
```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;  // we have to use the * dereference operator to get to the value in i before we can use the += operator.
}
```

## Using an Enum to Store Multiple Types
```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```
- When the vector gets dropped, all of its contents are also dropped, meaning the integers it holds will be cleaned up.
- Vector si dropped using '//' before the vector.

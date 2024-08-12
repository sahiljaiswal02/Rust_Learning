// Creating a new vector
let v: Vec<i32> = Vec::new();  
let v = vec![1, 2, 3];

// Updating a Vector
let mut v = Vec::new();  // We use push to add the elements in the vector
v.push(5);  
v.push(6);
v.push(7);
v.push(8);

// Reading Elements of Vectors
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];  // Indexing method
println!("The third element is {third}");

let third: Option<&i32> = v.get(2);  //Get method
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}


// Example:
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);

// Iterating Over the Values in a Vector
let v = vec![100, 32, 57];
for i in &v {
    println!("{i}");
}


// We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;  // we have to use the * dereference operator to get to the value in i before we can use the += operator.
}

// Using an Enum to Store Multiple Types
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


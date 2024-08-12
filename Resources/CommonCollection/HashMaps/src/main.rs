use std::collections::HashMap;  //Using HashMap from the library

// Creating a New Hash Map
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// Accessing Values in a Hash Map
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
let team_name = String::from("Blue");// This program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>
let score = scores.get(&team_name).copied().unwrap_or(0); // Then unwrap_or to set score to zero if scores doesn’t have an entry for the key.
                                                          
//To print the key value pairs
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
for (key, value) in &scores {
    println!("{key}: {value}");  //Yellow: 50 Blue: 10
}

// Hash Maps and Ownership
let field_name = String::from("Favorite color");
let field_value = String::from("Blue");
let mut map = HashMap::new();
map.insert(field_name, field_value);// field_name and field_value are invalid at this point

// Updating a Hash Map

// Overwriting a Value
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{scores:?}");  //{"Blue": 25}

// Adding a Key and Value Only If a Key Isn’t Present
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.entry(String::from("Yellow")).or_insert(50);  // Checks weather yellow key is present or not, if not then it will add key and value for yellow: 50
scores.entry(String::from("Blue")).or_insert(50); // Checks the same for blue but blue is already present so it just move forward
println!("{scores:?}"); // {"Yellow": 50, "Blue": 10}


// Updating a Value Based on the Old Value
// Example: Counts the occurrence of word
let text = "hello world wonderful world";

let mut map = HashMap::new();
for word in text.split_whitespace() {  //The split_whitespace method returns an iterator over subslices, separated by whitespace, of the value in text
    let count = map.entry(word).or_insert(0);  //It checks if the word is already present in the pair or not and if not it will add the word 
    *count += 1; // If the word is present then it will increase the count by 1 
}
println!("{map:?}");  //{"world": 2, "wonderful": 1, "hello": 1}

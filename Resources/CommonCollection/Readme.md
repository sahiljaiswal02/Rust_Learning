# Common Collections
> Rust's standard library offers powerful and versatile data structures known as collections.

> Collections store their data on the heap, allowing for dynamic resizing.

> Collections can be mutable or immutable depending on how you declare them
## 1. Vector ( `Vec<T>` )
- Description:
  > A Vec<T> is a growable list of values of type T stored contiguously in memory.
- Use Cases:
  > Ideal for scenarios where you need to store a variable number of items and frequently need to add or remove elements.
- Operations:
  > Supports indexing, iteration, appending, and removing elements.
#### Example:
```rust
let mut numbers = Vec::new();
numbers.push(1);
numbers.push(2);
```

## 2. String ( `String` )
- Description:
  > A String is a heap-allocated, growable string of UTF-8 bytes.
- Use Cases:
  > Useful for managing and manipulating text data dynamically.
- Operations:
  > Supports concatenation, slicing, and modification of text.
#### Example:
```rust
let mut greeting = String::from("Hello");
greeting.push_str(", world!");
```
## 3. HashMap ( `HashMap<K, V>` )
- Description:
  > A HashMap<K, V> stores key-value pairs where keys are unique and associated with values.
- Use Cases:
  > Ideal for scenarios where you need to quickly look up data associated with a specific key.
- Operations:
  > Supports inserting, retrieving, and removing key-value pairs.
#### Example:
```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert("Team A", 50);
scores.insert("Team B", 30);
```

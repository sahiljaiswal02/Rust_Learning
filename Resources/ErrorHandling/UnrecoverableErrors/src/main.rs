// Unrecoverable Errors with panic!

fn main() {
    panic!("crash and burn");  //panic will end the program
}

fn main() {
    let v = vec![1, 2, 3];

    v[99];  // Attempting to access an element beyond the end of a vector, which will cause a call to panic!
}

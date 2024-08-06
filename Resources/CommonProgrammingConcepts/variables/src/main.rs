//Imutable Variables
fn Imutable() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6; // As x is not mutable, It will give compile time error
    println!("The value of x is: {x}");
}

//Mutable Variables
fn Mutable() {
    let mut x = 5; // mut is used to make a variable mutable
    println!("The value of x is: {x}");
    x = 6; // Now x is mutable
    println!("The value of x is: {x}");
}

// Constants
fn Constants() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;  //Constants are Immutable
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");
}

//Shadowing
fn Shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); //When that scope is over, the inner shadowing ends
    }

    println!("The value of x is: {x}");
}

//Tuple
fn Tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}

fn Tuple1() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}

//array
fn Array() {
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}

fn Array1() {
    let arr = [3; 5]; //let a = [3, 3, 3, 3, 3];
}

fn main() {
    Imutable();
    Mutable();
    Constants();
    Shadowing();
    Tuple();
    Tuple1();
    Array();
    Array1();
}

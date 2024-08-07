//Using basic function
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}
fn area(width: u32, height: u32) -> u32 {
    width * height
}
//The area function is supposed to calculate the area of one rectangle, but the function we wrote has two parameters, and it’s not clear anywhere in our program that the parameters are related.


//Using Tuple
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
// We would have to keep in mind that width is the tuple index 0 and height is the tuple index 1. This would be even harder for someone else to figure out and keep in mind if they were to use our code

//Using Structs
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)  // we want to borrow the struct rather than take ownership of it
    );
}
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
//This gives the proper difference in the parameters and are related.

//Adding Useful Functionality with Derived Traits
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);  //error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`

}

//Adding Attribute 
#[derive(Debug)]  // we add the outer attribute #[derive(Debug)] just before the struct definition
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}"); // rect1 is Rectangle { width: 30, height: 50 }
}

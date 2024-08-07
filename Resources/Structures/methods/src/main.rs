//Finding area of the rectangle using Method
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {    //impl block are called associated functions
    fn area(&self) -> u32 {  //we use &self instead of rectangle: &Rectangle, &self here for the same reason we used &Rectangle
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

//Using method with condition
impl Rectangle {
    fn width(&self) -> bool { //give a method the same name as one of the struct’s fields
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

//Methods with More Parameters
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

impl Rectangle {
    fn area(&self) -> u32 {  //To calculate the area
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {  //To check if rect1 can hold rect2 and return bool
        self.width > other.width && self.height > other.height
    }
}

//Associated Functions
impl Rectangle {
    fn area(&self) -> u32 {  //To calculate the area
        self.width * self.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main(){
    let sq = Rectangle::square(3);
    println!("The area of square is {}", sq.area());
}

//Multiple impl Blocks
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

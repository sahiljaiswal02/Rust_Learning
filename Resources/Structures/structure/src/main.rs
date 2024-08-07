
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
//Imutable
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}

//Mutable
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}

//Return Struct User
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

//Another Way
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,  //email field and the email parameter have the same name, we only need to write email rather than email: email.
        sign_in_count: 1,
    }
}

// Creating instance from another instance
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,  //Using user1's active field
        username: user1.username,  //Using user1's username field
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,  //Using user1's sign_in_count field
    };
}

fn main() {
    // --snip--
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1  //The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
    };
}

// Using Tuple Structs Without Named Fields to Create Different Types
// Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// Unit Structs
struct AlwaysEqual;
fn main() {
    let subject = AlwaysEqual;
}
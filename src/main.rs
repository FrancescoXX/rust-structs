// Define a struct called User with four fields: name, email, is_active, and age.
struct User {
    name: String,
    email: String,
    is_active: bool,
    age: u8,
}

fn main() {
    // Create an instance of the User struct and assign it to the user variable.
    let user = User {
        name: String::from("Jhon Doe"),
        email: String::from("doe@mail.com"),
        is_active: true,
        age: 25,
    };

    // Access the name field of the user struct using dot notation.
    println!("User name: {}", user.name);
}

// Define a struct called User with four fields: name, email, is_active, and age.
struct User {
    name: String,
    email: String,
    is_active: bool,
    age: u8,
}

fn main() {
    // Make the struct mutable by using the mut keyword.
    let mut user = User {
        name: String::from("Jhon Doe"),
        email: String::from("doe@mail.com"),
        is_active: true,
        age: 25,
    };

    // Change the value of the name field.
    user.name = String::from("Francesco");
    println!("User name: {}", user.name);
}

// Define a struct called User with four fields: name, email, is_active, and age.
struct User {
    name: String,
    email: String,
    is_active: bool,
    age: u8,
}

fn main() {
    // Create a function that returns a struct instance.
    let user = create_user(String::from("Jhon Doe"), String::from("doe@mail"),);

    println!("User name: {}", user.name);
}

// Define a struct called User with four fields: name, email, is_active, and age.
struct User {
    name: String,
    email: String,
    is_active: bool,
    age: u8,
}

fn main() {
    // Create an instance of the User struct using another instance with the struct update syntax.
    let user1 = User {
        name: String::from("Jhon Doe"),
        email: String::from("doe@mail.com"),
        is_active: false,
        age: 40,
    };

    let new_user = User {
        name: String::from("Francesco"),
        email: user1.email,
        is_active: user1.is_active,
        age: user1.age,
    };

    println!("User name: {}", new_user.name);
}

// Define a tuple struct called Color.
struct Color(i32, i32, i32);
// Define a tuple struct called Point.
struct Point(i32, i32, i32);

fn main() {
    // Create instances of tuple structs and access their fields using dot notation.
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Black color: {}, {}, {}", black.0, black.1, black.2);
    println!("Origin: {}, {}, {}", origin.0, origin.1, origin.2);
}

// Define a unit-like struct called User.
#[derive(Debug)]
struct User;

fn main() {
    // Create an instance of the unit-like struct User.
    let user = User;
    println!("User: {:?}", user);
}

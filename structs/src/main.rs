fn main() {
    // Defining a struct
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // Using a struct after defining it
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername1234"),
        active: true,
        sign_in_count: 5,
    };

    // Changing the value of a mutable struct instance
    user1.email = String::from("newemail@example.com");
    println!(
        "Print them for no reason: {}, {}, {}",
        user1.username, user1.sign_in_count, user1.active
    );

    // Creating instances from other instances using Struct Update syntax
    let user2 = User {
        email: String::from("user2@example.com"),
        username: String::from("user202534"),
        ..user1
    };

    // Tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

// Ownership and Functions

fn main() {
    // creating a String from a string literal
    // this kind of String can grow and change (unlike normal string literals)
    let s = String::from("hello");

    takes_ownership(s); // move s into the function, can’t use s anymore

    //let v = s;

    let x = 5;
    /// simple integer
    makes_copy(x); // x is copied, not moved, so we can still use it

    //let _y = x;

    let s1 = gives_ownership(); // Function gives_ownership() gives us a String — we store it in s1
    let s2 = String::from("hello"); // Create a new String and store it in s2

    // Move s2 into the function takes_and_gives_back()
    // That function returns the String, which we store in s3
    let s3 = takes_and_give_back(s2);
}
// x goes out of scope here. s was already moved, so nothing happens
// Here, s3 goes out of scope and gets dropped (memory freed)
// s2 doesn’t get dropped here because it was moved into the function
// s1 also goes out of scope and gets dropped

fn takes_ownership(some_string: String) {
    println!("{}", some_string); // prints the string
}
// some_string is dropped here, memory freed

fn makes_copy(some_int: i32) {
    println!("{}", some_int); // prints the number
}
// nothing special here, simple/scalar types are copied

// Returning Values and Scope

// This function creates a String and gives (returns) ownership to whoever calls it
fn gives_ownership() -> String {
    let some_string = String::from("hello"); // Create a String inside the function
    some_string // Return the String — ownership moves to the caller
}

// This function takes ownership of a String and then gives it back (returns it)
fn takes_and_give_back(a_string: String) -> String {
    a_string // Return the same String, moving ownership back to the caller
}

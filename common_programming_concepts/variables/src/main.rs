fn main() {

    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

// Shadowing 
    let s = 5;
    let s = s + 1;
    let s = s * 2;
    let s = "Shadowing allows you do this, change an integer to a string";
    println!("The value of s is: {}", s);
}

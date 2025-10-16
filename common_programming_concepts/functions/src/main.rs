fn main() {
    // statements and expressions in a function body
    let x = 6; // This is a statement

    // This is an expression
    let y = {
        let x = 3;
        x + 1
    }; // Expressions do not end with semicolon so, at this point it becomes a statement
    println!("The value of y is: {}", y);

    another_function(10, 12); // argument

    let fn_five = five();
    println!("The value of fn_five is: {}", fn_five);

    let b = plus_one(5);
    println!("The value of b is: {}", b);
}

fn another_function(x: i32, y: i32) {
    // functions with parameters
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// function(s) with return values
fn five() -> i32 {
    5
}

fn plus_one(b: i32) -> i32 {
    b + 1
}

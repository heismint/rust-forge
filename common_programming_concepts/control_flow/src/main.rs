fn main() {
    // if expressions
    let number = 7;

    if number < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    // handling multiple conditions with else if
    let num = 6;

    if num % 4 == 0 {
        println!("Number is divisible by 4");
    } else if num % 3 == 0 {
        println!("Number is divisible by 3");
    } else if num % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3 or 2");
    }

    // usinf if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}

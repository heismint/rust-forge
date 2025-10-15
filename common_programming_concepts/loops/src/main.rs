fn main() {
    
    // loop
    loop{
        println!("Hello, world!");
        break;
    }

    // returning values from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);

    // conditional loops with while
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");

    // using a for loop for the example above
    for number in (1..4).rev(){
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");

    // looping through a collection with while
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 { // this approach is error prone, could cause the program to panic
        println!("The value is: {}", a[index]);
        
        index = index + 1;
    }

    // looping through a collection with for
    let b = [10, 20, 30, 40, 50];

    for element in b.iter() { // safer approach
        println!("The value is: {}", element);
    }
}

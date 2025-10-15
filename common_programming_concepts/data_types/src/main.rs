fn main() {

    // Scalar types(a data type family in RUST)
    // Floating point types
    let _x = 2.0; //f64
    let _y: f32 = 3.0; //f32

    // Numeric operations
    let _sum = 5 + 10; // addition

    let _difference = 95.5 - 4.3; // subtraction

    let _product = 4 * 40; // multiplication

    let _quotient = 6 / 2; // division

    let _remainder = 4 % 3; // remainder

    //Boolean type
    let _t = true;

    let _f: bool = false; // with explicit type annotation

    // Character type
    let _c = 'z';
    let _z = 'z';
    let _love = 'l';

    // Compound types(another data type family in RUST)
    // Tuple type
    let tup: (i32, f64, u8) = (500, 6.6, 6);
    let (x,y,z) = tup; // This is called destructuring.

    // accessing a tuple value with a period(.)
    let five_one_hundred = tup.0;

    // Array type 
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // accessing array elements
    let first = a[0];
    let second = b[1];
    println!("The value of y is: {}", y);
}

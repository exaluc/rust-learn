fn main() {
    // Floating-Point Types
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32


    // Numeric Operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    // The Boolean Type
    let t = true;

    let f: bool = false; // with explicit type annotation

    // The Character Type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tupl = (500, 6.4, 1);

    let (x, y, z) = tupl;

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // The Array Type
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    // Here, i32 is the type of each element. After the semicolon, the number 5 indicates the array contains five elements.
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // The array named a will contain 5 elements that will all be set to the value 3 initially.
    let a = [3; 5];

    // Accessing Array Elements
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
        

}

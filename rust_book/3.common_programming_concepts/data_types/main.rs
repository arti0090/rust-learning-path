fn main() {
    // SCALARS

    let a = 2;
    let b: i8 = 3;

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // COMPOUNDS

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    // will return 6.4
    println!("The value of y is: {y}");

    // direct access to tuple vars
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // array (usefull when data is allocated on stack)
    // usefull when number of elements do not change
    let a = [1, 2, 3, 4, 5];

    // with type and number of elements
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // specify the array of 5 elements - each is 3
    let a = [3; 5]; // = [3, 3, 3, 3, 3];

    // array access 
    let first = a[0];
    let second = a[1];
}
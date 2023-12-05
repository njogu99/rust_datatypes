fn main() {
    //floating-point numbers
    let a = 2.0; // f64

    let b: f32 = 3.0; // f32

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

    //boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    //character type
    let c = 'z';
    let d: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';


    //COMPOUND TYPES
    //tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    let g: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = g.0;

    let six_point_four = g.1;

    let one = g.2;

    //arrays
    let h =  ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    let first = h[0];
    let second = h[1];

    println!("X is: {a}");
    println!("Y is: {b}");
    println!("The sum is: {sum}");
    println!("The difference is: {difference}");
    println!("The product is: {product}");
    println!("The quotient is: {quotient}");
    println!("The truncated is: {truncated}");
    println!("The remainder is: {remainder}");
    println!("The bool 1 is: {t}");
    println!("The bool 2 is: {f}");
    println!("C is: {c}");
    println!("Z is: {d}");
    println!("The heart eyed cat is: {heart_eyed_cat}");
    println!("The value of x in the tuple is: {x}");
    println!("The value of y in the tuple is: {y}");
    println!("The value of z in the tuple is: {z}");
    println!("The values in the tuple are: {five_hundred}, {six_point_four}, {one}");
    println!("The first value of the array is: {first}");
    println!("The second value of the array is: {second}");



}

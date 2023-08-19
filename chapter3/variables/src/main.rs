fn main() {

    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // Shadowing
    println!("\nSHADOWING \n");

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    // let mut spaces = "   ";
    // spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    // Data Types
    println!("\nDATA TYPES \n");

    let guess: u32 = "42".parse().expect("Not a Number!");
    // let guess = "42".parse().expect("Not a Number!");
    println!("guess {}", guess);

    // Floating-Point Types
    println!("\nFLOATING-POINT \n");

    let x = 2.0; // f64
    println!("Floating-point f64 {}", x);

    let y :f32 = 3.0; // f32
    println!("Floating-point f32 {}", y);

    // Numeric Operation
    println!("\nNUMERIC OPERATION \n");

    // addition
    let sum = 5 + 10;
    println!("sum {}", sum);

    // substraction
    let difference = 95.5 - 4.3;
    println!("difference {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("product {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient {}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("remainder {}", remainder);


    // Boolean Type
    println!("\nBOOLEAN TYPE \n");

    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("true: {}", t);
    println!("false: {}", f);

    // Character Type
    println!("\nCHARACTER TYPE \n");

    let c = 'z';
    let z = 'c';
    let heart_eye_cat = '😻';

    println!("letter c {}",c);
    println!("letter z {}",z);
    println!("icen cat {}",heart_eye_cat);

    // Tupla type
    println!("\nTUPLA TYPE \n");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of x: {}", x);
    println!("The value of y: {}", y);
    println!("The value of z: {}", z);

    let x :  (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("\nfive hundred {}", five_hundred);
    println!("six point four {}", six_point_four);
    println!("one {}", one);

    // Array type
    println!("\nARRAY TYPE \n");

    let a = [1, 2, 3, 4, 5];

    println!("a[0] {}", a[0]);
    println!("a[1] {}", a[1]);
    println!("a[2] {}", a[2]);
    println!("a[3] {}", a[3]);
    println!("a[4] {}", a[4]);

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    println!("\nmonths[0] {}",months[0]);

    // Declare a new array with signed 32 bits and five elements
    let a : [i32; 5] = [1,2,3,4,5];

    println!("first element {} = 1", a[0]);

    // Declare a new arreay with five elements just all the content is 3
    let a = [3;5];

    println!("a[0] {}", a[0]);
    println!("a[1] {}", a[1]);
    println!("a[2] {}", a[2]);
    println!("a[3] {}", a[3]);
    println!("a[4] {}", a[4]);

}

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

}

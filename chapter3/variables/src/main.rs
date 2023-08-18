fn main() {

    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // Shadowing

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    // let mut spaces = "   ";
    // spaces = spaces.len();


    // Data Types

    let guess: u32 = "42".parse().expect("Not a Number!");
    // let guess = "42".parse().expect("Not a Number!");

    // Floating-Point Types

    let xx = 2.0; // f64

    let yy :f32 = 3.0; // f32

}

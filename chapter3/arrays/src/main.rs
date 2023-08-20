fn main() {
    
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

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("\naccesing first element = {} ", first);
    println!("accesing second element = {} ", second);

}

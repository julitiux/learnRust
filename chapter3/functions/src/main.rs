fn main() {
    println!("Hello, world!");

    another_function();
    another_function1(5, 6);

    let five = five();

    println!("function five {}", five);

    let x = plus_one(5);

    println!("The value of x is: {}", x);

}

fn another_function() {
    println!("Another Function");
}

fn another_function1(x: i32, y: i32){
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: u32) -> u32 {
    x + 1
}

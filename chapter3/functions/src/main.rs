fn main() {
    println!("Hello, world!");

    another_function();
    another_function1(5);

}

fn another_function() {
    println!("Another Function");
}

fn another_function1(x: i32){
    println!("The value of x is: {}", x);
}

fn main() {
    println!("Hello, world!");

    another_function(11, 22);

    let x = plus_five(1);
    println!("x: {}", x);

    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}

fn another_function(x: i32, y: i32) {
    println!("another_function, x: {}, y: {}", x, y);
}

fn plus_five(x: i32) -> i32 {
    x + 5
}

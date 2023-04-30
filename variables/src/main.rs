// const MAX_POINTS: u32 = 100_000;


fn main() {

    println!("Hello, world!");

    let mut x  = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);

    let y = 5;
    // Shadowing 隐藏
    // Shadowing 的类型是可以被改变的。原来是String 后面可以是usize
    let y = y + 1;

    println!("y: {}", y);

    match_func();
}

fn match_func() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }
    // match number {
    //     number % 4 == 0 => {
    //         println!("number is divisible by 4");
    //     },
    //     number % 3 == 0 => {
    //         println!("number is divisible by 3");
    //     },
    //     number % 2 == 0 => {
    //         println!("number is divisible by 2");
    //     }, 
    //     {
    //         println!("number is not divisible by 4, 3 or 2");
    //     },
    // }
}

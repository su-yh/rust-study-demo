// 常量的声明与定义
const _MAX_POINTS: u32 = 100_000;


fn main() {

    println!("Hello, world!");

    let a = 5;
    println!("The value of a is {}", a);

    // a 是一个不可变的量。不允许对它进行二次赋值。
    // a = 6;

    let mut x  = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);

    let y = 5;
    // Shadowing 隐藏
    // Shadowing 的类型是可以被改变的。原来是String 后面可以是usize
    let y = y + 1;

    println!("y: {}", y);

    // shadowing 可以改变原来的变量的类型
    let spaces = "    ";
    let spaces = spaces.len();

    println!("spaces: {}", spaces);

    match_func();

    array_demo();
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
    match number {
        num if num % 4 == 0 => {
            println!("number is divisible by 4");
        },
        num if num % 3 == 0 => {
            println!("number is divisible by 3");
        },
        num if num % 2 == 0 => {
            println!("number is divisible by 2");
        }
        _ => {
            println!("number is not divisible by 4, 3 or 2");
        },
    }
}

// 数组下标越界在编译时不会报错，只有在运行的时候才会检查，并报错，最终引起panic
fn array_demo() {
    let a = [1, 2, 33, 4, 5];
    println!("a[0]: {}", a[0]);

    // 声明一个数组，指定这个数组元素的类型是i32 类型，同时指定数组长度为5 
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    println!("b[0]: {}", b[0]);

    // 声明并初始化一个数组，这个数组的所有值都是0，并且一共有5 个元素
    let c = [0; 5];
    println!("c[0]: {}", c[0]);
}

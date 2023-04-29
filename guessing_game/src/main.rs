// prelude
use std::cmp::Ordering;
use rand::Rng; // trait

fn main() {
    println!("猜数！");

    // i32  u32  i64
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("神秘数字是: {}", secret_number);

    loop {
        println!("猜测一个数");

        // 使用mut 标识该变量是可变的。否则不可变
        let mut guess = String::new();

        std::io::stdin().read_line(&mut guess)
        // .expect 处理如果读取发生错误的问题。
        .expect("无法读取行");
        // io::Result Ok, Err

        // 定义一个u32 类型的变量
        // rust 允许 同名的变量，将前面的同名的变量隐藏。
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        } ;


        println!("你猜测的数是：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

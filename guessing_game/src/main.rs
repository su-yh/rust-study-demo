// 为什么必须要引入它，在不引入它的时候会报错？
use rand::Rng;

fn main() {
    println!("猜数！");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("神秘数字是：{}", secret_number);

    loop {
        println!("猜测一个数！");

        let mut guess = String::new();

        // io::Result Ok, Err 两个变体
        let res = std::io::stdin().read_line(&mut guess).expect("读取失败！");

        println!("您的输入是：{}", guess);
        println!("返回值结果: {}", res);

        // shadow 隐藏同名的变量，一般用在类型变化
        // let guess: i32 = guess.trim().parse().expect("解析失败，你的输入 不是数字");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

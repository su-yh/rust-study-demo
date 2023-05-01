

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    println!("Hello, world!");

    let user1 = User {
        email: String::from("abcd@126.com"),
        username: String::from("name"),
        active: true,
        sign_in_count: 455,
    };

    // 使用user1 的某些字段来创建一个新的user 实例
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    // 可以有简化的语法
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername"),
        // 余下的还没有赋值的字段，全部从user1 中取
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,  // 简写的方式，字段名与变量名相同时
        username,
        active: true,
        sign_in_count: 0,
    }
}


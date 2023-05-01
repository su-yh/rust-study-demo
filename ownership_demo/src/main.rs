// Rust 中的所有权：ownership

// 所有 权规则
// 每个值都 有一个变量，这个变量是该 值的所有 者
// 每个值同时只能有一个所有 者
// 当所有者超出作用域 时，该 值将被删除

// move 移动仅仅是相对于栈内存的。
// 与引用相关的都是借用

use std::string;

// 也就是说通过赋值操作就会发生所有权的转移
fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    demo_move();

    let s = String::from("Hello World");

    take_ownership(s);

    // s 已经移动到了函数里面，它的所有权已经转移，所以在这里s 已经失效，我们不能再 使用它。
    // println!("{}", s);

    let x = 5;
    makes_copy(x);
    println!("x: {}", x);

    let s1 = gives_ownership();

    let s2 = String::from("hello");

    // s2 在这里所有权已经发生了转移，所以后面不能再使用这个s2 变量了。
    let s3 = takes_and_gives_back(s2);

    // 这里使用s2 报错了，因为它的所有权已经转移，在这里它已经失效了。
    // println!("s2: {}", s2);
    println!("s1: {}, s3: {}", s1, s3);

    other_demo();

    demo02();

    demo03();

    demo04();

    demo05();

    demo06();

    demo07();

    demo08();

    demo09();

    
}



fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_number: i32) {
    println!("{}", some_number);
}

fn demo_move() {
    let s1 = String::from("hello");
    let s2 = s1;

    // s1 已经 被移动，它不可再 使用，因为它已经失效了。
    // println!("s1: {}", s1);
    println!("s2: {}", s2);
}

fn other_demo() {
    let s1 = String::from("hello");

    let (s2, len) = calulate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calulate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn demo02() {
    let s1 = String::from("Hello");
    // &s1 创建一个对s1 的引用，并不取得其所有权。
    // 把引用作为函数参数的这个行为叫做借用
    let len = calculate_length(&s1);

    // 因为s1 并没有被移动，它只是被借用了，所以它的所有权还存在，
    // 所以这里就还可以继续使用s1 这个变量
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn demo03() {
    let mut s1 = String::from("Hello");
    // &s1 创建一个对s1 的引用，并不取得其所有权。
    // 把引用作为函数参数的这个行为叫做借用
    let len = calculate_length03(&mut s1);

    // 因为s1 并没有被移动，它只是被借用了，所以它的所有权还存在，
    // 所以这里就还可以继续使用s1 这个变量
    println!("The length of '{}' is {}.", s1, len);
}

// 可变的引用作为参数类型，就可以对该 参数进行修改了。
fn calculate_length03(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}


fn demo04() {
    let mut s = String::from("Hello");

    println!("s: {}", &mut s);

    // 可变引用有一个重要的限制：在特定作用域内，对某一块数据，只能有一个可变的引用。
    // let s1 = &mut s;
    // let s2 = &mut s;
}

fn demo05() {
    let mut s = String::from("Hello");

    println!("s: {}", &mut s);

    // 可变引用有一个重要的限制：在特定作用域内，对某一块数据，只能有一个可变的引用。
    {
        // 可以在不同作用域里面对同一个变量创建可变引用。
        let s1 = &mut s;
        println!("s1: {}", s1);
    }
    let s2 = &mut s;
    println!("s2: {}", s2);
}

fn demo06() {
    let mut s = String::from("Hello");
    let r1 = &s;
    let r2 = &s;
    let s1 = &mut s;

    // 不可以同时拥有不可变引用 与可变引用。但是可以同时拥有多个可变引用 。
    // println!("{} {} {}", r1, r2, s1);
}

fn demo07() {
    let mut s = String::from("Hello world");
    let worldIndex = first_world(&s);

    s.clear();

    println!("{}", worldIndex);
}

fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}


// 在 demo07 中，虽然使用了可变引用 s 来进行清空操作，但是由于该引用的作用域仅限于函数 first_world07 内部，因此不会与函数 demo07 中的不可变引用 s 发生冲突。
// 而在 demo08 中，函数 first_world08 返回一个指向字符串 s 的切片。切片的本质是对原始数据的引用，当使用切片时，其实也就相当于使用原始数据的一个视图。
// 根据 Rust 的借用规则，在存在不可变引用时，不能同时存在可变引用。因此，在函数 demo08 使用 s.clear() 对字符串 s 进行清空操作时，就违反了 Rust 的借用规则，导致程序出现了错误。
// 因此，为了避免这个问题，我们需要尽可能地避免同时存在可变和不可变引用，并且必须在编写代码时遵守 Rust 借用规则。
fn demo08() {
    let mut s = String::from("Hello world");
    let worldIndex = first_world08(&s);

    s.clear();

    // println!("{}", worldIndex);
}

fn demo0008() {
   
    let mut s = String::from("Hello world");
    let worldIndex = first_world08(&s);

    println!("{}", worldIndex);

    s.clear();

    // println!("{}", worldIndex);
}

fn first_world08(s_param: &String) -> &str {
    let bytes = s_param.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s_param[..i];
        }
    }

    &s_param[..]
}

fn demo09() {
    let my_string = String::from("Hello world");
    let wordIndex = first_world09(&my_string[..]);

    let my_string_listeral = "hello world";
    let wordIndex = first_world09(my_string_listeral);
}

fn first_world09(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}



// Copy trait, 可以用于像整数这样完全存放 在stack 上面的类型
// 如果一个类型实现了Copy 这个trait，那么旧的变量在赋值后仍然可用
// 如果一个类型或者该 类型的一部分实现了drop trait，那么rust 不允许让它再 去实现copy trait了

use std::vec;

fn main() {
    println!("Hello, world!");

    demo01();

    demo02();

    demo03();
}

fn demo01() {
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];
    let third = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }


    let mut v = Vec::new();
    v.push(1);

}

fn demo02() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    // v.push(6);

    println!("The first element is {}", first);
}

fn demo03() {
    let mut v = vec![100, 32, 58];

    for ele in &v {
        println!("{}", ele);
    }

    for ele in &mut v {
        *ele += 1;
    }

    for ele in &v {
        println!("{}", ele);
    }

}

fn demo04() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(3.1415),
    ];

    // println!("{}", row);
}


fn main() {
    demo01();
    demo02();
    demo03();
    demo04();
    demo05();
}

fn demo01() {
    let mut scores = std::collections::HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
}

fn demo02() {
    let teams = vec![String::from("Blue"), String::from("Yello")];
    let intial_scores = vec![10, 50];

    let scores: std::collections::HashMap<_, _> = teams.iter().zip(intial_scores.iter()).collect();

    // println!("{}", scores);
}

fn demo03() {
    let field_name = String::from("favorite color");
    let field_value = String::from("Blue");

    // 使用引用，所有权不会丢失，后续还可以使用。
    let mut map2 = std::collections::HashMap::new();
    map2.insert(&field_name, &field_value);
    println!("{}: {}", field_name, field_value);

    let mut map = std::collections::HashMap::new();
    map.insert(field_name, field_value);

    // 所有权发生了移动
    // println!("{}: {}", field_name, field_value);
}

fn demo04() {
    let mut scores = std::collections::HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 遍历HashMap
    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }
}

fn demo05() {
    let mut scores = std::collections::HashMap::new();

    scores.insert(String::from("Blue"), 10);

    // scores.entry(String::from("Yellow")).or_insert(50);
    let e = scores.entry(String::from("Yellow"));
    println!("{:?}", e);
    e.or_insert(50);    // 如果存在，返回对应的V 的一个可变引用，不存在则插入

    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

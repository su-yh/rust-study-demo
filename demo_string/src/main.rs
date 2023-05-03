// rust 有三种看待字符串的方式：
// 字节
// 标题值
// 字形簇（最接近所谓的“字母”） 

fn main() {
    println!("Hello, world!");

    demo01();

    demo02();

    demo03();

    demo04();
}

fn demo01() {
    let mut s = String::new();
    s.push_str("foo ");
    let s1 = String::from("initial contents");
    s.push_str(&s1);
    s.push('.');


    println!("{}", s);
    println!("{}", s1);

}

fn demo02() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");

    // +: 使用了类似签名方法：fn add(self, s: &str) -> String {...}
    // 所以第一个是被移动了
    let s3 = s1 + &s2;

    println!("{}", s3);
    // println!("{}", s1);  // s1 在上面被移动了。
    println!("{}", s2);
}

fn demo03() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let s3 = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s3);
}

fn demo04() {
    let w = "中文数字";

    println!("len(): {}", w.len());

    for ele in w.chars() {
        println!("{}", ele);
    }

    for ele in w.bytes() {
        println!("{}", ele);
    }
}
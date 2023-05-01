fn main() {
    let some_number = Some(5);
    let some_string = Some("hello");

    let absent_number: Option<i32> = None;


    let x = 5;
    let y = Some(8);


    println!("{}", x + match y {
        Some(i) => i,
        None => 0,
    });

    if let Some(i) = y {
        println!("y is {}", i);
    } else {
        println!("others");
    }

}
fn main() {
    let rect = Rectangle {
        width: 30,
        length: 50,
    };
    println!("{}", area(&rect));
    println!("{}", rect.area());
}

struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
    
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}

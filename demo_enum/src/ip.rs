fn main() {
    println!("Hello, world!");
    let four = IpAddrKind::V4(127,0,0,1);
    let six = IpAddrKind::V6(String::from("::1"));

    route(four);
    route(six);
}

enum IpAddrKind {
    // 变体: ipv4
    V4(u8, u8, u8, u8),
    // 变体：ipv6
    V6(String),
}

fn route(ip_kind: IpAddrKind) {}

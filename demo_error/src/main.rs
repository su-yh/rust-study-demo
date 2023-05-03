// rust 中的错误处理

use std::{io::Read, net::IpAddr};

fn read_username_from_file() -> Result<String, std::io::Error> {
    let f = std::fs::File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}


// ? 表达式，把错误向上传递给调用者，让调用者来处理异常s。
// ? 类型只能用于返回值为Result 或者Option.
fn read_username_from_file_02() -> Result<String, std::io::Error> {
    let mut f = std::fs::File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_03() -> Result<String, std::io::Error> {
    let mut s = String::new();
    std::fs::File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}


fn main() {
    // demo01();
    // demo02();
    // demo03();
    // demo04();
    demo05();
}

fn demo01() {
    let f = std::fs::File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Error opening file {:?}", error);
        }
    };
}

fn demo02() {
    let f = std::fs::File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // 文件打开出错了，但是这里对不同的错误进行不同的处理
            // 如果是文件不存在，则创建这个文件
            std::io::ErrorKind::NotFound => {
                // 创建这个文件
                let fc = std::fs::File::create("hello.txt");
                let f = match fc {
                    Ok(f) => f,
                    Err(e) => panic!("Error opening file {:?}", e),
                };
                f   // 这一行还挺重要，因为是最后一行语句，它将会作为结果返回给前面的调用者
            }
            // 如果是其他错误，就直接panic 结束
            other_error => panic!("Error opening the file: {:?}", other_error),
        },
    };
}


fn demo03() {
    let f = std::fs::File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            std::fs::File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Error opening file {:?}", error);
            })
        } else {
            panic!("Error opening the file: {:?}", error);
        }
    });
}

fn demo04() {
    let result = read_username_from_file();

    let f = match result {
        Ok(f) => f,
        Err(error) => panic!("error {}", error),
    };
}

fn demo05() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();

    println!("{}", home);
}


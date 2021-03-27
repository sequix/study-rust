use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

// main 函数只能返回 () 或者 Result<T, E>
fn main() -> Result<(), Box<dyn Error>> {
    // not recoverable
    // panic!("crashed");

    // recoverable
    // let f = match File::open("./test") {
    //     Ok(f) => f,
    //     Err(e) => panic!("Problem opening the file: {:?}", e),
    // };

    // let f = match File::open("./test") {
    //     Ok(f) => f,
    //     Err(e) => match e.kind() {
    //         ErrorKind::NotFound => match File::create("./test") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("problem createing the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };

    // let f = File::open("./test").unwrap_or_else(|e| {
    //     if e.kind() == ErrorKind::NotFound {
    //         File::create("./test").unwrap_or_else(|e| {
    //             panic!("creating: {:?}", e);
    //         })
    //     } else {
    //         panic!("opening: {:?}", e);
    //     }
    // });

    // unwrap() returns Result if succes, panic! if not
    // let f = File::open("./test").unwrap();

    // expect() 和 unwrap() 一样，但可以自定义错误信息
    // let f = File::open("./test").expect("failed to open ./test");

    // ? 操作符可以用在返回 Result 的函数上，其作用和 第一个 read_username_form_file 一样
    let f = File::open("./test")?;
    Ok(())
}

// 返回错误
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("./test");

//     let mut f = match f {
//         Ok(f) => f,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// 效果同上，用的是 rust 提供的 ? 语法糖写出来的
// ? 会调用 错误类型的 from 函数，用以将错误转换为返回类型需要的错误类型
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("./test")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// 同上，更短
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut s = String::new();
//     File::open("hello.txt")?.read_to_string(&mut s)?;
//     Ok(s)
// }

// 同上，更短
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("./test")
}

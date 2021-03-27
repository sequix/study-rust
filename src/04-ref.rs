fn main() {
    // x = &i32
    let x = &1;
    // y = &i32
    let ref y = 2;
    // z = i32
    let z = 3;

    f1(x);
    f1(y);
    // f1(z) 类型不匹配
    // f1(z);

    // expected `i32`, found `&i32`
    // f2(x);

    // expected `i32`, found `&i32`
    // f2(y);

    f2(z);

    let s = String::from("test");
    f3(s);
    // f3(s) 已经 move 了 s 到 f3 中
    // println!("s after f3: {}", s);

    let s2 = String::from("test2");
    f4(&s2);
    println!("s2 after f4: {}", s2);
}

fn f1(x: &i32) {
    println!("f1: {}", x);
}

fn f2(ref x: i32) {
    println!("f2: {}", x);
}

fn f3(ref x: String) {
    println!("f3: {}", x);
}

fn f4(ref x: &String) {
    println!("f4: {}", x);
}

// ref x: doesn't have a size known at compile-time
// fn f5(ref x: str) {
//     println!("f5: {}", x);
// }

fn f6(ref x: &str) {
    println!("f6: {}", x);
}

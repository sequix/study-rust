fn main() {
    // Stirng 是具体的 字符串类
    // str 是 String 切片，所以只有 &str 形式
    // string literal 本质也是 &str，只不过是对 .rodata 的引用

    let s = "hello".to_string();
    println!("{}", s);

    let s = String::from("hi");
    println!("{}", s);

    let s: String = "also this".into();
    println!("{}", s);

    // cat
    let mut msg = s + " world";
    println!("{}", msg);

    msg.push_str(" else");
    println!("{}", msg);

    msg.push('无');
    println!("{}", msg);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);

    // rust 字符串 以 utf8  存储
    let sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
    assert_eq!("💖", sparkle_heart);
    let bytes = sparkle_heart.into_bytes();
    assert_eq!(bytes, [240, 159, 146, 150]);

    // 由于 String 内部是 vec<byte>，所以不支持 indexing
    // 仅支持 indexing 到 utf8 边界
    let hello = "Здравствуйте";
    // 非 utf8 边界，会 panic
    // let s = &hello[0..1];
    let s = &hello[0..4];

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}

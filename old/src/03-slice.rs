fn main() {
    let mut s = String::from("0123 56789");
    println!("{}", first_word(&"123 123".to_string()));
    println!("{} {}", &s[..3], &s[5..]);

    // let word = first_word_2(&s);

    s.clear();

    println!("{}", s);

    for e in (1..4).rev() {
        println!("e {}", e);
    }
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// fn first_word_2(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }
//     return &s[..];
// }

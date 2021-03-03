// 每个 reference 都有 lifetime，注意是 reference。

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

fn main() {
    // let r;
    // // use r before you give r a value
    // // println!("r: {}", r);
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r);

    // let str1 = String::from("abcd");
    // {
    //     let str2 = "xyz";
    //     let rst = longest(str1.as_str(), str2);

    //     // lifetime(rst) = lifetime(str2) <= lifetime(str1)
    //     println!("longer str {}", rst);
    // }

    // let str1 = String::from("abcd");
    // let rst;
    // {
    //     let str2 = String::from("xyz");
    //     rst = longest(str1.as_str(), str2.as_str());
    // }
    // // lifetime(str2) < lifetime(str1)
    // // lifetime(rst) > lifetime(str2)
    // println!("longer str {}", rst);

    let str1 = String::from("abcd");
    let rst;
    {
        //  string literal and its type is &'static str.
        // A string literal is a string slice that is statically allocated,
        // meaning that it’s saved inside our compiled program, and exists for the entire duration it runs.
        // let str2: &'static str = "xyz";
        let str2 = "xyz";
        rst = longest(str1.as_str(), str2);
    }
    // lifetime(str2) < lifetime(str1)
    // lifetime(rst) > lifetime(str2)
    println!("longer str {}", rst);

    let novel = String::from("Call me Ishmael, Some year ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// 无法确定 x、y 谁返回，也就无法确定 x、y 的 lifetime
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// lifetime annotation 不影响 reference 的实际的 lifetime，只是方便为了 borrow checker 确定 lifetime
// 下述的含义是，x、y、返回值 有相同的 lifetime
// 'a 的实际 lifetime，是x，y 中的较小值，函数签名这样写，也就限制了返回值的 lifetime 要小于等于 x、y lifetime 较小的
// NB：这个 lifetime 分析是 rust 编译器静态做的！！！
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// lifetime 价值在于避免 dangling reference，而对于传进函数的参数，rust 编译器无法静态分析其 lifetime，因此也就无法确定 ref 是否有效

// 必须要有一个 lifetime parameter，因为 &str 是对一个 String 片段的引用，rustc 无法知道被引用 String lifetime，也就不知道 返回值&str 的有效范围
// fn longest3(x: &str, y: &str) -> &str {
//     let rst = String::from("rellay long string");
//     rst.as_str()
// }

// 当未通过参数限定 lifetime a，则返回值必须返回函数创建的 ref
// fn longest2<'a>(x: &str, y: &str) -> &'a str {
//     let rst = String::from("rellay long string");
//     // rst 在函数结束时，lifetime 就结束了，所以不能返回
//     rst.as_str()
// }

//  ImportantExcerpt 实例的 lifetime 小于等于 part 的 lifetime
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// rustc 可以 infer 一些 lifetime pattern
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// rustc 依次使用下述规则尝试推导 lifetime
// 1.每个参数分配不同的lifetime，即 fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
// 2.只有一个参数时，将返回值 lifetime 设置为参数的 lifetime，即 fn foo<'a>(x: &'a i32) -> &'a i32
// 3.多个参数，但存在 &self 或 &mut self，则 fn foo<'a>(&'a self) -> &'a i32

// Lifetime names for struct fields always need to be declared after the impl keyword and then used after the struct’s name, because those lifetimes are part of the struct’s type.
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // fn announce_and_return_part<'a,'b>(&'a self, announcement: &'b str) -> &'a str {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

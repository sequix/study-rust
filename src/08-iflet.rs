enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`).
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case.
        println!("Didn't match a number. Let's go with a letter!");
    }

    let c = Foo::Qux(100);
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    } else {
        println!("c is not one hundred");
    }

    let a = Foo::Bar;
    // Foo::Bar 没实现 PartialEq，所以会失败
    // if Foo::Bar == a {
    // 而这个是在做解构，不是在调用 trait，所以成功
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    let mut optional = Some(0);
    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
            },
            _ => { break; }
        }
    }

    // while let 也支持解构
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}
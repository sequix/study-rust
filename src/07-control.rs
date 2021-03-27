#![allow(unreachable_code)]

fn main() {
    let n = 20;

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            10 * n
        } else {
            println!(", and is a big number, halve the number");
            n / 2
        };
    println!("{}", big_n);

    'outer: loop {
        println!("Entered the outer loop");
        '_inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }
        println!("This point will never be reached");
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);

    for n in 1..3 {
        println!("{}", n);
    }
    for n in 1..=3 {
        println!("{}", n);
    }

    /////////////////

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() { // iter borrow immutable
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);

    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() { // iter_mut borrow mutable
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() { // into_iter move
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // println!("names: {:?}", names);
}

// An attribute to hide warnings for unused code.
#![allow(dead_code)]

use std::mem;

struct Point {
    x: f32,
    y: f32,
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Creates a type alias
type Operation = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            // Self is also a type alias
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    let _an_integer = 5i32; // Suffix annotation
    let mut _inferred_type = 12; // Type i64 is inferred from another line
    _inferred_type = 4294967296i64;

    println!("One million is written as {}", 1_000_000u32);

    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // All elements can be initialized to the same value
    let xs: [i32; 5] = [0; 5];
    println!("array debugable {:?}, length {}", xs, xs.len());
    println!("mem usage {} bytes", mem::size_of_val(&xs));
    println!("slice debugable {:?}", &xs[0..3]);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };
    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point {
        x: top_edge,
        y: left_edge,
    } = point;
    println!("top_edge: {}, left_edge: {}", top_edge, left_edge);

    let _x = Operation::Subtract;

    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

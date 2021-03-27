#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value }
    }
}

#[derive(Debug, PartialEq)]
struct EvenNumber {
    value: i32,
}

// 只有在本作用域的 trait，才能使用该 trait 的函数，所以用 trait，必须 use
use std::convert::{TryFrom, TryInto};

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value & 1 == 0 {
            Ok(EvenNumber { value })
        } else {
            Err(())
        }
    }
}

impl ToString for EvenNumber {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

use std::str::FromStr;
impl FromStr for EvenNumber {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: i32 = s.parse().unwrap();
        if value & 1 == 0 {
            Ok(EvenNumber { value })
        } else {
            Err(())
        }
    }
}

fn main() {
    let decimal = 65.4321_f32;

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);
    println!(
        "size of `x` in bytes: {}",
        std::mem::size_of_val(&character)
    );

    let num = Number::from(23);
    println!("{:?}", num);

    let n = 42;
    let num2: Number = n.into();
    println!("{:?}", num2);

    // 要使用 assert_eq 必须用 PartialEq
    assert_eq!(EvenNumber::try_from(23), Err(()));
    assert_eq!(EvenNumber::try_from(24), Ok(EvenNumber { value: 24 }));
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    println!("{}", result.unwrap().to_string());

    let result: EvenNumber = "6".parse().unwrap();
    println!("{:?}", result);
}

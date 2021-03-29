use std::num::ParseIntError;

// 自定义 Result 的别名，类似 io::Result
type AliasedResult<T> = Result<T, ParseIntError>;

// Use the above alias to refer to our specific `Result` type.
fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    // first_number_str.parse::<i32>().and_then(|first_number| {
    //     second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    // })

    // unwrap panics，? returns 
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}

// Here, the alias again allows us to save some space.
fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

// Option<T> 和 Result<T,E>
//  都支持 unwrap，默认 None/Err 时的行为都是 panic
//  都支持 map/and_then
// fn main() -> Result<(), ParseIntError> {
    // let number_str = "10 ";
    // let number = match number_str.parse::<i32>() {
    //     Ok(number)  => number,
    //     Err(e) => return Err(e),
    // };
    // println!("{}", number);
    // Ok(())
// }

fn main()  {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}


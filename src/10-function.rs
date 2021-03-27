
fn is_divisible_by(n: u32, x: u32) -> bool {
    n % x == 0
}

// Functions that "don't" return a value, actually return the unit type `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

struct Point {
    x: f64,
    y: f64,
}

// `Pair` owns resources: two heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // impl 中的函数叫 method，不在 impl 的叫 function
    fn destroy(self) {
        // Destructure `self`
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
    }
}

// ! 标明函数从不返回
// 即返回 "从不返回" 这一概念
// 此概念不能被赋予变量，除非 #![feature(never_type)]
fn foo() -> ! {
    panic!("This call never returns.");
}

fn main() {
    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
    // Error! Previous `destroy` call "consumed" `pair`
    //pair.destroy();

    let is_odd = |n: u32| {
        n & 1 == 1
    };
    // Imperative approach
    let upper = 100;
    let mut acc = 0;
    // Iterate: 0, 1, 2, ... to infinity
    for n in 0.. {
        let n_squared = n * n;

        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)                             // All natural numbers squared
            .take_while(|&n_squared| n_squared < upper) // Below upper limit
            .filter(|&n_squared| is_odd(n_squared))     // That are odd
            .fold(0, |acc, n_squared| acc + n_squared); // Sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            let addition: u32 = match i%2 == 1 {
                true => i,
                false => continue, // 此处的 match 的返回值其实是 ! (从不返回) (loop{} 的返回值也是这个)
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));

    // #![feature(never_type)]
    // let x: ! = panic!("This call never returns.");
    // println!("You will never see this line!");
}

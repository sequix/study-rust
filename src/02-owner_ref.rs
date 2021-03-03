//fn main() {
//    f(3);
//
//    let cond = true;
//    let number = if cond { 1 } else { 0 };
//
//    // 值要一样
//    // !let number = if cond { 1 } else { "not ok" };
//
//    let mut cnt = 3;
//    
//    loop {
//        if cnt <= 0 {
//            break;
//        }
//        println!("{}!", cnt);
//        cnt -= 1;
//    }
//
//    let a = [4, 3, 2, 1];
//    for n in a.iter() {
//        println!("y {}", n);
//    }
//
//    // not include 4
//    for n in (1..4).rev() {
//        println!("x {}", n);
//    }
//}
//
//fn f(x: i32) {
//    let y = {
//        let z = x * x;
//        z / 2
//    };
//    println!("f got {}, x*x/2 = {}", x, y);
//}

fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

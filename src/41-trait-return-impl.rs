use std::iter;
use std::vec::IntoIter;

// 注意这里的返回值的大小是明确的，所以是合法的
fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// 这个函数和上面的一样，只不过返回的是一个的 trait
// 这个返回值的大小是编译器来推导出来的
fn combine_vecs(
    v: Vec<i32>,
    u: Vec<i32>,
) -> impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// 闭包都是有唯一的匿名类型的，所以返回闭包必须用 impl 语法
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| { x + y };
    closure
}

// lifetime 是必须的，rustc 不会为返回 impl 推导 lifetime
fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item=i32> + 'a {
    numbers
        .iter()
        .filter(|x| x > &&0)
        .map(|x| x * 2)
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    println!("all done");

    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);

    let v4 = &vec![1, 2, 3, 4, 5, 6];
    let xi = double_positives(v4);
    for (index, item) in xi.enumerate() {
        println!("{:?} {:?}", index, item);
    }
}

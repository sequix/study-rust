/*
闭包相关的 trait，用来声明闭包为参数用：
Fn: the closure captures by reference (&T)
FnMut: the closure captures by mutable reference (&mut T)
FnOnce: the closure captures by value (T)

注意使用 FnOnce 肯定能用 FnMut，使用 FnMut 肯定能用 Fn，即限制程度从大到小：
FnOnce > FnMut > Fn
因为 move 可以，&mut 一定可以，&mut可以，&更可以
*/

fn apply<F>(f: F) where F: FnOnce() {
    f();
}

// 闭包参数必须指定泛型类型
fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
    f(3)
}

// 返回闭包时候，返回的是闭包的实例，所以返回类型中要写 impl
fn create_fn() -> impl Fn() {
    let text = String::from("Fn");
    // 由于函数返回时，闭包所引用的变量都会被释放，所以闭包必须用 move，来拿走 ownership
    // 保证函数返回后闭包不被释放
    move || println!("This is a {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = String::from("FnMut");

    move || println!("This is a {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = String::from("FnOnce");

    move|| println!("This is a {}", text)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));

    // 可以传递函数给闭包
    fn triple(x:i32) -> i32 { x * 3 }
    println!("3 tripled: {}", apply_to_3(triple));

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();
    fn_plain();
    fn_mut();
    fn_once();
}

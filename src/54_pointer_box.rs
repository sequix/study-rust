// 标准库的 Box<T> 实现了 deref 和 drop :

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Deref Trait 支持 * 解引用操作
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *(y.deref())); // 和上语句执行的流程一样，注意 * 还是需要的

    let _z = *y;
    assert_eq!(5, *y); // 由于 i32 实现了 Copy，所以这里发生复制，没有 move

    let x = String::from("not copy");
    let y = MyBox::new(x);

    // assert_eq!("not copy", x); // x 的ownership 已经给 y 了
    assert_eq!("not copy", *y);

    // let z = *y; // 这里发生了 move，所以下句编译报错
    // assert_eq!("not copy", *y);

    let z = &*y; // 对 heap 上变量的引用，注意 lifetime 比较还是在生效的
    assert_eq!("not copy", *y);
    assert_eq!("not copy", z);

    // let mut z = String::from("");
    // {
    //     let x = String::from("not copy");
    //     let y = MyBox::new(x);
    //     z = &*y; // y 会在该 block 后被释放，编译器在比较 y 和 z 的 lifetime 时发现，'z > 'y，所以编译失败
    // }
    // assert_eq!("not copy", z);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(m.deref().deref()); // 上语句实际执行这个，第二次 deref 调用是从 String 上进行的
    hello(&(*m)[..]); // String *m -> &String &(*m) -> &str &(*m)[..]

    /*
    From &T to &U when T: Deref<Target=U>
        fn deref(&self: T) -> &U
    From &mut T to &mut U when T: DerefMut<Target=U>
        fn deref_mut(&mut self) -> &mut U
    From &mut T to &U when T: Deref<Target=U>
        fn deref_mut(&mut self) -> &U (并没有这个 trait method，它实际走的是 rustc 的 borrow 规则)
     */
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
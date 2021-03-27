fn main() {
    // let add_one_v2 = |x: u32| {x + 1};
    // let add_one_v3 = |x| {x + 1};
    // let add_one_v4 = |x| x + 1;

    let c1 = |x| x;
    // 第一次使用 closure 时，rustc 会推导出 closure 的类型
    let s = c1(String::from("test"));
    // let n = c1(2);

    // 闭包可以访问其所在上层作用域的变量，但注意，这回让 rust 程序在内存中保留被 闭包引用的变量
    // let x = 4;
    // let equal_to_x = |z| z == x;
    // let y = 4;
    // assert!(equal_to_x(y));

    // 相对的，函数不能在其所在的作用域外获得变量
    // let x = 4;
    // fn equal_to_x(z: i32) -> bool {
    //     z == x
    // }
    // let y = 4;
    // assert!(equal_to_x(y))

    // 根据闭包函数对 environment variable 的使用方式，rust 实现下述几个 trait：
    // FnOnce 将环境变量的ownership拿到了闭包内部，所以这样的闭包只能执行一次，所以叫 Once (所有的闭包都实现了 Once，因为至少能执行一次)
    // FnMut 会改变环境变量，并且是通过 &mut 改变的环境变量
    // Fn 不会改版环境变量，或者是通过 & 访问的环境变量

    let x = vec![1, 2, 3];
    // move 关键字指明，闭包需要把引用的环境变量 x 的ownership 拿进闭包内
    // 注意 move 指定了闭包的捕获方式，不是使用方式，所以不影响其 trait，还可能是 FnMut、Fn 的
    let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x); // 因为 main 已经不再具有 x 的 ownership
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
    // assert!(equal_to_x(y));  // 上一次调用中 x 已经被释放了 (used after move)
}

fn add_one_v1(x: u32) -> u32 {
    x + 1
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calc: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calc: T) -> Cacher<T> {
        Cacher {
            calc: calc,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calc)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

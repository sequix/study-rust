// Non-copyable types.
struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

// 为所有 U 类型实现 DoubleDrop<T> 这个 trait  (so meta programming...)
impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null  = Null;

    empty.double_drop(null);

    //empty;
    //null;
    // ^ TODO: Try uncommenting these lines.

    let s = String::from("你甚至可以在一个 String 上使用");
    let s2 = String::from("test");
    s.double_drop(s2);
}

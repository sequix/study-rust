use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T where
    Option<T>: Debug { // 代表 Option<T> 这个类型实现了 Debug
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];
    vec.print_in_option();
}

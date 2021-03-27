fn main() {
    // fn() -> i32
    let _one = || 1;
    fn function(i: i32) -> i32 { i + 1 }

    let color = String::from("green");
    // println! 对 color 的访问方式是 &，所以 print 对 color 的捕获方式也是 &
    let print = || println!("`color`: {}", color);
    print();
    // 一个作用域内可以存在多个 &
    let _reborrow = &color;
    print();
    let _color_moved = color;

    let mut count = 0;
    // count 被修改，所以 inc 使用 &mut 访问 count
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    // Call the closure using a mutable borrow.
    inc();
    // 作用域内出现过 &mut ，不能再出现其它引用
    // let _reborrow = &count;
    inc();
    // 可以再次 &mut 借走 count，但之后 inc 就不能再借用 count 了
    let _count_reborrowed = &mut count;
    // inc();

    use std::mem;
    let movable = Box::new(3);
    // drop 需要 T，所以 movable 直接被 move ，不能再被使用
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };
    // println!("{:?}", movable); // ERR
    consume();
    // consume(); // ERR

    // | 前的 move 关键字直接拿走 owner
    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));
}
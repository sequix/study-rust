use std::rc::Rc;
use std::rc::Weak;
// 单线程的引用计数指针，clone时计数加一，没有内存复制发生，drop时计数减一，计数为0，销毁 rc 和 持有对象

fn main() {
    let rc_examples = "Rc examples".to_string();
    {
        println!("--- rc_a is created ---");

        let rc_a: Rc<String> = Rc::new(rc_examples);
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

        {
            println!("--- rc_a is cloned to rc_b ---");

            let rc_b: Rc<String> = Rc::clone(&rc_a);
            println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

            // Two `Rc`s are equal if their inner values are equal
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));

            // We can use methods of a value directly
            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);

            println!("--- rc_b is dropped out of scope ---");
        }

        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

        println!("--- rc_c is downgraded created ---");
        let rc_c: Weak<String> = Rc::downgrade(&rc_a); // drop 调用不受弱引用计数影响
        println!("Strong Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
        println!("Weak Reference Count of rc_a: {}", Rc::weak_count(&rc_a));

        println!("--- rc_a is dropped out of scope ---");
    }

    // Error! `rc_examples` already moved into `rc_a`
    // And when `rc_a` is dropped, `rc_examples` is dropped together
    // println!("rc_examples: {}", rc_examples);
    // TODO ^ Try uncommenting this line
}

struct ToDrop;

// 析构函数由 Drop trait 提供
impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    let _x = ToDrop;

    // 对于实现了 Copy trait 的类型，= 代表复制，没有 move 发生
    let x = 5u32;
    let y = x;
    println!("x is {}, and y is {}", x, y);

    // 在 move 的时候，可以改变其 mutability
    let immutable_box = Box::new(5u32);
    println!("immutable_box contains {}", immutable_box);
    // *Move* the box, changing the ownership (and mutability)
    let mut mutable_box = immutable_box;
    println!("mutable_box contains {}", mutable_box);
    *mutable_box = 4;
    println!("mutable_box now contains {}", mutable_box);

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }
    let person = Person {
        name: String::from("Alice"),
        age: 20,
    };
    // name = String, age = &u8，只有部分的 move 发生了
    let Person { name, ref age } = person;
    println!("The person's age is {}", age);
    println!("The person's name is {}", name);
    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);
    println!("The person's age from person struct is {}", person.age);
}



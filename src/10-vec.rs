fn main() {
    // 0 init
    let mut v: Vec<i32> = Vec::new();

    for n in 0..3 {
        v.push(n);
    }

    assert_eq!(v.len(), 3);
    assert_eq!(v[0], 0);

    assert_eq!(v.pop(), Some(2));
    assert_eq!(v.len(), 2);

    for n in &v {
        print!("{} ", n);
    }
    println!();

    let v = vec![1, 2, 3];
    assert_eq!(v.len(), 3);

    let v = vec![0; 3];
    assert_eq!(v, [0, 0, 0]);

    // same as above, but potentially slower
    let mut v = Vec::with_capacity(3);
    v.resize(3, 0);
    assert_eq!(v, [0, 0, 0]);

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!();

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for n in &v {
        print!("{} ", n);
    }
    println!();

    // 1 indexing
    let v = vec![0, 2, 4, 6];
    // panic
    // println!("{}", v[9]);

    match v.get(9) {
        Some(n) => println!("n = {}", n),
        None => println!("not exists"),
    }

    /*
        error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
      --> src/main.rs:53:5
       |
    52 |     let first = &v[0];
       |                  - immutable borrow occurs here
    53 |     v.push(8);
       |     ^^^^^^^^^ mutable borrow occurs here
    54 |     println!("first element {}", first);
       |                                  ----- immutable borrow later used here
        */
    // 注意，此处的 borrow 是发生在 v 上的，而不是 v 的元素上的
    // let first = &v[0];
    // v.push(8);
    // println!("first element {}", first);

    // 2 slicing
    let v = vec![1, 2, 3, 4];
    read_slice(&v);

    // let u: &[usize] = &v;
    let u: &[_] = &v;
    read_slice(u);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for x in &row {
        println!("{:?}", x);
    }
}

fn read_slice(s: &[usize]) {
    for n in s {
        print!("{} ", n);
    }
    println!();
}

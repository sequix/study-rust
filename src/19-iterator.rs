fn main() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    // 注意返回的是 immutable ref
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    for val in v1.iter() {
        println!("Got: {}", val);
    }

    // let mut v1_iter_mut = v1.iter_mut();

    // assert_eq!(v1_iter_mut.next(), Some(&mut 1));
    // assert_eq!(v1_iter_mut.next(), Some(&mut 2));
    // assert_eq!(v1_iter_mut.next(), Some(&mut 3));
    // assert_eq!(v1_iter_mut.next(), None);

    // sum 的文档在哪看？
    // sum 怎么定义的，因为返回的只是一个迭代器的 trait，不应该有 sum 这样的具体方法和其对应？
    let sum: i32 = v1.iter().sum();

    println!("sum: {}", sum);

    let v2: Vec<i32> = vec![1, 2, 3];
    let v3: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v3, vec![2, 3, 4]);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
}

// 程序会默认引入哪些 package?
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
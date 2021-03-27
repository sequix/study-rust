/*
pub trait Iterator {
    type Item;

    fn any<F>(&mut self, f: F) -> bool where
        F: FnMut(Self::Item) -> bool {}
}
*/

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`. Destructure to `i32`.
    println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
    // `into_iter()` for vecs yields `i32`. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2));
    // 已经被 move 过了
    // println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2));

    let a = [1, 2, 3];
    let mut iter = a.iter();
    assert_eq!(iter.find(|&&x| x == 2), Some(&2));
    assert_eq!(iter.next(), Some(&3));

    let a = [1, 2, 3, 4];
    let mut iter = a.iter();
    assert_eq!(iter.position(|&x| x >= 2), Some(1));
    assert_eq!(iter.next(), Some(&3));

    let a = ["lol", "NaN", "2", "5"];
    let first_number = a.iter().find_map(|s| s.parse().ok());
    assert_eq!(first_number, Some(2));
}
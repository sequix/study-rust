struct Point { x: i32, y: i32, z: i32 }

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    println!("Point has coordinates: ({}, {}, {})",
             borrowed_point.x, another_borrow.y, point.z);

    // 不可此时 &mut，因为后面使用了 &，&期待着不变的值
    // let mutable_borrow = &mut point;

    println!("Point has coordinates: ({}, {}, {})",
             borrowed_point.x, another_borrow.y, point.z);

    let mutable_borrow = &mut point;

    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // 此时被 mutable borrow，所以不能再 immutable borrow
    // let y = &point.y;

    // 因为 println! 拿的是 immutable borrow 所以，这里也不能输出
    // println!("Point Z coordinate is {}", point.z);

    println!("Point has coordinates: ({}, {}, {})",
             mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

    let new_borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
             new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);
}

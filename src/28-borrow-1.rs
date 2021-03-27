use std::ops::Add;

// This function takes ownership of a box and destroys it
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// This function borrows an i32
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {
    let mut boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // immutable borrow 可以多次
    borrow_i32(&boxed_i32);
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_mut_i32_0: &mut i32 = &mut boxed_i32;
        let _ref_to_mut_i32_1: &mut i32 = &mut boxed_i32;
        *_ref_to_mut_i32_1 = 12;
        let _ref_to_mut_i32_2: &mut i32 = &mut boxed_i32;
        *_ref_to_mut_i32_2 = 23;

        let _ref_to_i32: &i32 = &boxed_i32;

        // 已经有 immutable borrow 时，不能再 &mut
        // let _ref_to_mut_i32_3 = &mut boxed_i32;


        // 因为 boxed_i32 被 _ref_to_i32 借用了，所以不能被销毁
        // eat_box_i32(boxed_i32);

        borrow_i32(_ref_to_i32);
        // `_ref_to_i32` goes out of scope and is no longer borrowed.
    }

    // `boxed_i32` can now give up ownership to `eat_box` and be destroyed
    eat_box_i32(boxed_i32);
}

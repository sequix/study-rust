// lifetime of x will not exceed 'a,
// namely, lifetime of x will not exceed of four (variable in main)
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// Because the lifetime is never constrained, 'a defaults to `'static`.
fn failed_borrow<'a>() {
    let _x = 12;

    // ERROR: `_x` does not live long enough
    let y: &'a i32 = &_x;
    // _x will be dropped at the end of this function, but y has a longer lifetime
    // , namely, 'static. So you cannot assign a reference with shorter to one with longer lifetime.
}

fn main() {
    // Create variables to be borrowed below.
    let (four, nine) = (4, 9);

    // Borrows (`&`) of both variables are passed into the function.
    print_refs(&four, &nine);

    failed_borrow();
}

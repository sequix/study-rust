struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

// impl of Val
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// impl of GenVal for a generic type `T`
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

// ERR: duplicate definitions for `value`
// impl GenVal<f32> {
//     fn value(&self) -> &f32 {
//         &self.gen_val
//     }
// }

fn main() {
    struct S;
    struct GenericVal<T>(T);

    // impl of GenericVal where we explicitly specify type parameters:
    impl GenericVal<f32> {} // Specify `f32`
    impl GenericVal<S> {} // Specify `S` as defined above

    impl<T> GenericVal<T> {}

    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());
}

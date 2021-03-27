// extern crate rary; // May be required for Rust 2015 edition or earlier

mod lib17;

fn main() {
    lib17::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    lib17::indirect_access();
}

// bash 多路径的语法
// use std::{cmp::Ordering, io};

// use io & io::Write
use std::io::{self, Write};

// use all pub members of std::collections
use std::collections::*;

mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

// use crate::front_of_house::hosting;
// use front_of_house::hosting;
pub use self::front_of_house::hosting;
pub use self::front_of_house::hosting::add_to_waitlist as atw;

pub fn eat_at_restaurant() {
    // hosting::add_to_waitlist();
    atw();
}

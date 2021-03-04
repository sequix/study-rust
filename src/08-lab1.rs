// packge 包含 多个 crate
// crate 包含多个 mod，对应单个源文件，是 rustc 编译、连接、版本管理的最小单位，rustc 会为 crate 自动创建同名的 mod
// mod 包含实际的 rust 元素，struct、enum、fn 等等
// path 引用 mod 中的 rust 元素的方法，类比 posix 路径
//    crate = /
//    self  = .
//    super = ..
//    <mod> = <directory>
//    * = *
//    {self,a} = {,a}
//    {a,b,c}  = {a,b,c}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}

        mod back_of_house {
            fn fix_incorrect_order() {
                cook_order();
                // like ../
                super::serve_order();
            }
            fn cook_order() {}
        }
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // abs path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    // front_of_house::hosting::add_to_waitlist();
    self::front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

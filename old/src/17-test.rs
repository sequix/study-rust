// cargo test --help    # 传参 --help 给 cargo test
// cargo test -- --help # 传参 --help 给 单测程序
// cargo test -- --test-threads=1  # 默认并发执行所有测试，这样则为串行
// cargo test -- --show-output     # 同样展示成功测试的输出
// cargo test key                  # 执行单测名称包含 key 的测试
// cargo test --include-ignored    # 也执行带 #[ignored] anntations 的单测
// cargo test --ignored            # 只执行带 #[ignored] anntations 的单测

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smllaer() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    #[ignore]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
        assert_ne!(5, add_two(2));
    }

    #[test]
    #[ignore]
    fn greeting_contains_name() {
        let result = greeting("sequix");
        assert!(
            result.contains("sequix"),
            "Greeting did not contain name, value was `{}`",
            "sequix",
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_msg() {
        Guess::new(200);
    }

    // 返回 error 时，不能使用 should_panic
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}

pub fn add_two(a: i32) -> i32 {
    a+2
}

pub fn greeting(name: &str) -> String {
    format!("Hello !")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}


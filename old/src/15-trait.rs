pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> String;

    fn summarize_with_default(&self) -> String {
        format!("(read more... {})", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// we can implement a trait on a type only if either the trait or the type is local to our crate
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content"),
    };

    println!("{}", article.summarize());
    println!("{}", article.summarize_with_default());
    notify(&article);

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let s = 3.to_string();
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize())
// }

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}

// pub fn notify(item: &(impl Summary + Display)) {
// pub fn notify<T: Summary + Display>(item: &T) {

/*
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
*/

fn returns_summarizable() -> impl Summary {
    NewsArticle {
        headline: String::from("headline2"),
        location: String::from("location2"),
        author: String::from("author2"),
        content: String::from("content2"),
    }
}

// 不能用不同的实际类型
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

/*
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
*/

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

/*
/// A trait for converting a value to a `String`.
///
/// This trait is automatically implemented for any type which implements the
/// [`Display`] trait. As such, `ToString` shouldn't be implemented directly:
/// [`Display`] should be implemented instead, and you get the `ToString`
/// implementation for free.
///
/// [`Display`]: fmt::Display
#[stable(feature = "rust1", since = "1.0.0")]
pub trait ToString {
    /// Converts the given value to a `String`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let i = 5;
    /// let five = String::from("5");
    ///
    /// assert_eq!(five, i.to_string());
    /// ```
    #[rustc_conversion_suggestion]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn to_string(&self) -> String;
}

// 对所有实现了 Display 的类型 T，实现 ToString
/// # Panics
///
/// In this implementation, the `to_string` method panics
/// if the `Display` implementation returns an error.
/// This indicates an incorrect `Display` implementation
/// since `fmt::Write for String` never returns an error itself.
#[stable(feature = "rust1", since = "1.0.0")]
impl<T: fmt::Display + ?Sized> ToString for T {
    // A common guideline is to not inline generic functions. However,
    // removing `#[inline]` from this method causes non-negligible regressions.
    // See <https://github.com/rust-lang/rust/pull/74852>, the last attempt
    // to try to remove it.
    #[inline]
    default fn to_string(&self) -> String {
        use fmt::Write;
        let mut buf = String::new();
        buf.write_fmt(format_args!("{}", self))
            .expect("a Display implementation returned an error unexpectedly");
        buf
    }
}
*/

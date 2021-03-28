//fn invalid_output<'a>() -> &'a String { &String::from("foo") }
//  没有被限制，所以其实际为 'static
// 而返回值的 lifetime 仅在该函数内有效，所以错误

// ********************************************************

// rustc 依次使用下述规则尝试推导 lifetime
// 1.每个参数分配不同的lifetime，即 fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
// 2.只有一个参数时，将返回值 lifetime 设置为参数的 lifetime，即 fn foo<'a>(x: &'a i32) -> &'a i32
// 3.多个参数，但存在 &self 或 &mut self，则 fn foo<'a>(&'a self) -> &'a i32

// ********************************************************

// A struct with annotation of lifetimes.
#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

// Annotate lifetimes to impl.
impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self {
            x: &10,
        }
    }
}

// ********************************************************

use std::fmt::Debug;

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
/*
T: 'a: All references in T must outlive lifetime 'a.
T: Trait + 'a: Type T must implement trait Trait and all references in T must outlive 'a.
*/

fn print_ref<'a, T>(t: &'a T) where
    T: Debug + 'a {
    println!("`print_ref`: t is {:?}", t);
}

// ********************************************************

// 声明 'a >= 'b，所以 lifetime 较大的 first 可以返回 (coercion)
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 { first }

// 报错，rustc 的 lifetime 分析是在一个函数内发生的（大概是这样，要不为什么引入lifetime标记)
//fn choose_first<'a, 'b>irst: &'a i32, _: &'b i32) -> &'b i32 { first }

// ********************************************************

// 'static reference 有两种形式，字符串字面值, static, 常量

// static 可以在 unsafe 代码中被修改
static NUM: i32 = 18;
// const 总是不可被修改的
const NUM_CONST: i32 = 42;

fn coerce_static(_: &i32) -> &i32 {
    &NUM
}

fn give_me_a_static(n: &'static i32) -> &'static i32{
    const NUM_CONST2: i32 = 42;
    &NUM_CONST2
}

// ********************************************************

// 'static 作为 trait bound 时代表：参数不含有任何 非staitc 引用
// input does not contain any non-static references
fn print_it( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}

fn main() {
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);

    let x = 7;
    let ref_x = Ref(&x);
    print_ref(&ref_x);

    let first = 2; // Longer lifetime
    {
        let second = 3; // Shorter lifetime
        println!("{} is the first", choose_first(&first, &second));
    };

    {
        let lifetime_num = 9;
        let coerced_static = coerce_static(&lifetime_num);
        println!("coerced_static: {}", coerced_static);
    }
    println!("NUM: {} stays accessible!", NUM);
    give_me_a_static(&NUM_CONST);

    // 所有变量的都是有 lifetime 的，只不过非引用类型的lifetime是 'static
    // i is owned and contains no references, thus it's 'static:
    let i = 5;
    print_it(i);

    {
        let i2 = 12;
        print_it(i2);
    }

    // oops, &i only has the lifetime defined by the scope of
    // use_it(), so it's not 'static:
    // print_it(&i);
}


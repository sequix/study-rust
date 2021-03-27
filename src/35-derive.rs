/*
可以通过 #[derive] 继承的 trait：
Comparison traits: Eq, PartialEq, Ord, PartialOrd.
Clone, to create T from &T via a copy. 即令对象可以使用功能.clone()
Copy, to give a type 'copy semantics' instead of 'move semantics'.
Hash, to compute a hash from &T.
Default, to create an empty instance of a data type.
Debug, to format a value using the {:?} formatter.

Eq: 数学意义上的相等，必须满足 交换、传递、自反，默认是递归定义的(Eq要求所有对应Fields都是Eq的），递归的底部是 primitive type 的 Eq
PartialEq：满足交换、传递，所以自反 x==x 是确定二者的区别，默认是递归定义的(PartialEq要求所有对应Fields都是PartialEq的）

Ord:
total and asymmetric: exactly one of a < b, a == b or a > b is true; and
transitive, a < b and b < c implies a < c. The same must hold for both == and >.
默认成员从上到下的字母序列排列

PartialOrd:
asymmetry: if a < b then !(a > b), as well as a > b implying !(a < b); and
transitivity: a < b and b < c implies a < c. The same must hold for both == and >.
默认成员从上到下的字母序列排列

注意：NaN != NaN，不满足自反，所以对于浮点数只能使用 PartialEq 和 PartialOrd

hash：最终结果是所有 field 的 hash 的 hash

default：默认值是 field 的默认值
*/


#[derive(Default,Debug)]
struct SomeOption {
    foo: i32,
    bar: f32,
}

fn main() {
    let opt1: SomeOption = Default::default();
    let opt2 = SomeOption{ foo:42, ..Default::default() };
    println!("{:?} {:?}", opt1, opt2);
}
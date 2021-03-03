struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
    // 可以在 strcut 中存储 &，但需要考虑生命周期
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 0,
        active: false,
    }
}

struct Color(i32,i32,i32);

fn main() {
    let u1 = build_user(String::from("a@a"), String::from("name"));
 
    let u2 = User{
        email: String::from("x"),
        ..u1
    };

    println!("{}", u2.email);

    struct Point(i32, i32, i32);

    let c1 = Color(0, 1, 2);
    let p1 = Point(0, 0, 0);

    println!("{} {} {}", c1.0, p1.1, c1.2);



}

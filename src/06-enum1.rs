enum IpAddr {
    V4(u8,u8,u8,u8), 
    V6(String),
}

struct Color(i32,i32,i32);

enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(Color),
}

impl Message {
    fn call(&self) {
    }
}

fn main() {
    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("test"));
    m.call();

    let some_number = Some(5);
    let some_thing = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // !let sum = x + y;
}
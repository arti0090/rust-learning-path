enum IpAddrKind {
    V4(String),
    V6(String),
}

// Enum instances
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

// function that gets enum
fn route(ip_kind: IpAddrKind) {}

// create IP with string
let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));

// enum with different param values
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));

// standard package IPAddr https://doc.rust-lang.org/std/net/enum.IpAddr.html

// enum with variety of types
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// you can impl methods in enums
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
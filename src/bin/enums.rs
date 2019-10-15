
fn main() {

}

enum IpAddrKind {
    IPv4,
    IPv6,
}

fn create_enum() {
    let ip4 = IpAddrKind::IPv4;
    let ip6 = IpAddrKind::IPv6;
}

struct IpAddr {
    kind: IpAddrKind,
    addr: String,
}

fn create_struct_with_enum() {
    let loopback = IpAddr {
        kind: IpAddrKind::IPv4,
        addr: String::from("127.0.0.1"),
    };
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self)
    }
}

use crate::Message::Write;

fn call_message() {
    let msg = Write(String::from("hello"));
    msg.call();
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter: {:?}", state);
            25
        },
    }
}

fn get_plus_one() {
    match plus_one(Some(5)) {
        Some(result) => println!("{}", result),
        _ => ()
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        let i = x?;
        Some(i + 1)
    }
}
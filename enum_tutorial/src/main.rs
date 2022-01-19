#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddress {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn call(&self) {
        println!("call called");
    }
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    is_ip_address_v4(&four);
    is_ip_address_v4(&six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("home address is {:#?}", home);
    let four = IpAddress::V4(String::from("127.0.0.1"));
    println!("four address is {:#?}", four);

    let msg = Message::Quit;
    msg.call();

    let op: Option<i32> = Some(5);
    println!("option value is {:?}", op);

    let ret = match four {
        IpAddress::V4(s) =>  {println!("V4"); String::from("V4!")},
        IpAddress::V6(s) => {println!("V6"); String::from("V4!")},
    };
    println!("retvalue is {}",  ret);
    let coin = Coin::Penny;
    let cent = value_in_cents(coin);
    println!("cent is {}", cent);
    // println!("coin is {:?}", coin);
    let coin = Coin::Quarter(String::from("Alaska"));
    let cent = value_in_cents(coin);
    println!("cent is {}", cent);
    match_int(1);
    match_int(2);
}

fn is_ip_address_v4(kind: &IpAddrKind) {
}

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!("penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { 
            println!("State is {}", state);
            25
        },
    }
}

fn match_int(i: i32) {
    match i {
        1 => println!("one"),
        _ => println!("else one"),
    }
}
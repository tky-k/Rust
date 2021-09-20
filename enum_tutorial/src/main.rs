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
}

fn is_ip_address_v4(kind: &IpAddrKind) {
}
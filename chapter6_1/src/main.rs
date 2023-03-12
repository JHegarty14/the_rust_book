enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddrVariants {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // pass
    }
}

struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let ip4 = IpAddrKind::V4;
    let ip6 = IpAddrKind::V6;

    route(ip4);
    route(ip6);

    // kinda wrong
    let wrong_home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let wrong_loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // more concise
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // with variable member types
    let v_home = IpAddrVariants::V4(127, 0, 0, 1);
    let v_loopback = IpAddrVariants::V6(String::from("::1"));

    let m = Message::Write(String::from("message"));
    m.call();

    let x = 5;
    let y: Option<i32> = None;
    let z = match y {
        None => 0,
        Some(n) => n + x,
    };
    println!("z: {z}");
}

fn route(ip_kind: IpAddrKind) {
    
}

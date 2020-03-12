enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrKind2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr2 {
    kind: IpAddrKind2,
    address: String,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };


    let home2 = IpAddr2 {
        kind: IpAddrKind2::V4(1, 2, 3, 4),
        address: String::from("::1"),
    };

    let loopback2 = IpAddr2 {
        kind: IpAddrKind2::V6(String::from("::1")),
        address: String::from("::1"),
    };

    println!("Hello, world!");

    match loopback2.kind {
        IpAddrKind2::V6(value) => println!("loopback2.kind is: {}", value),
        _ => println!("Something else"),
    }
}

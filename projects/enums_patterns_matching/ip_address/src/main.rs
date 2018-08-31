enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

fn route(ip_route: IpAddrKind){}

route(IpAddrKind::V4);
route(IpAddrKind::V6);


// IpAddr with struct
struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("..1"),
};
// end with struct


// IpAddr with enum
enum IpAddr{
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("12.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
// end with enum

// enum with diferrent data types
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127,0,0,1);
let home = IpAddr::V6(String::from("::1"));

//end with different data types


fn main() {
    println!("Hello, world!");
}

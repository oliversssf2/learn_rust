enum IpAddr {
    V4(String), 
    V6(String),
}

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    route(home);
    
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
}

fn route(ip_kind: IpAddr) {}
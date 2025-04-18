// Can put what types the data needs to be right in the enum
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Example of a method using an enum
fn route(ip_kind: IpAddr) {
    println!("Routing!");
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}

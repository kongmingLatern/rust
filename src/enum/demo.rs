enum IpAddKind {
    V4,
    V6,
}
struct IpAdd {
    kind: IpAddKind,
    address: String,
}
// 可以简写为
enum IpAdd {
    V4(u8, u8, u8, u8),
    V6(String),
}
fn main() {
    // let home = IpAdd {
    //     kind: IpAddKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // let loopback = IpAdd {
    //     kind: IpAddKind::V6,
    //     address: String::from("::1"),
    // };

    // 可以简写为
    let home = IpAdd::V4("127, 0, 0, 1");
    let home = IpAdd::V6(String::from("::1"));
}
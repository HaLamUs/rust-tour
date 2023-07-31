enum IpAddrKind {
  V4(u8, u8, u8, u8), 
  V6(String),
}

enum Message {
  Quit,
  Move { x: i32, y: i32}, // struct 
  Write(String),
  ChangeColor(i32, i32, i32),
}

fn main() {
  let localhost = IpAddrKind::V4(127, 0, 0, 1);
}

fn route(ip_kind: IpAddrKind) {}
enum IpAddrKind {
  V4, 
  V6,
}

struct IPAddr {
  kind: IpAddrKind,
  address: String,
}



fn main() {
  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;

  let localhost = IPAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1")
  };
}

fn route(ip_kind: IpAddrKind) {}
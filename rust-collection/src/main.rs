fn main() {
    let a = [1, 2, 3];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(1);
    v.push(1);

    {
      let v2 = vec![1, 2, 3];
    } // out here v2 will drop
}

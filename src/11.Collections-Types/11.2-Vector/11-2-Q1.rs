fn main() {
    let arr: [u8; 3] = [1, 2, 3];
    let v = Vec::from(arr);
    is_vec(&v);
    let v = vec![1, 2, 3];
    is_vec(&v);
    let v = vec!(1, 2, 3);
    is_vec(&v);
    let mut v1 = Vec::new();
    for i in &v {
        v1.push(*i)
    }
    is_vec(&v1);
    assert_eq!(format!("{:?}",v), format!("{:?}",v1));
    println!("Success!")
}
fn is_vec(v: &Vec<u8>) {}
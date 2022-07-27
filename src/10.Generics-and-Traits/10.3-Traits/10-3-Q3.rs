use std::ops;
fn main() {
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));
}
fn multiply<T: ops::Mul<Output = T>>(x: T, y: T) -> T {
    x * y
}
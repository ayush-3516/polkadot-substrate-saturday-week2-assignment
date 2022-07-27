#[derive(Debug)]
struct Number {
    value: i32,
}
fn main() {
    let num = Number::from(30);
    assert_eq!(num.value, 30);
    let num: Number = 30.into();
    assert_eq!(num.value, 30);
    println!("Success!")
}
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}
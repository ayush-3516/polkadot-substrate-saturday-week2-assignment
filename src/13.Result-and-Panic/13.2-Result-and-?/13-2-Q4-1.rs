use std::num::ParseIntError;
fn main() {
    assert_eq!(add_two("4").unwrap(), 6);
    println!("Success!")
}
fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
   n_str.parse::<i32>().map(|num| num +2)
}
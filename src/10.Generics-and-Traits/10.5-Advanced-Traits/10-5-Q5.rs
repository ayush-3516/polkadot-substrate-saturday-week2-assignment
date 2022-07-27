use std::fmt;
struct Pretty(String);
fn main() {
    let w = Pretty("hello".to_string());
    println!("w = {}", w);
}
impl fmt::Display for Pretty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.0.clone() + ", world")
    }
}
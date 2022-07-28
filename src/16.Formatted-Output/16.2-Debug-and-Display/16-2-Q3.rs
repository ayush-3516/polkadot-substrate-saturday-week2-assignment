use std::fmt;
struct Structure(i32);
struct Deep(Structure);
fn main() {    
    println!("Now {:?} will print!", Deep(Structure(7)));
}
impl fmt::Debug for Deep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0.0)
    }
}
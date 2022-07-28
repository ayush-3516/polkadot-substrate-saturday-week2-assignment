struct ImportantExcerpt<'a> {
    part: &'a str,
}
fn main() {}
impl<'a> ImportantExcerpt<'a> {
    fn level(&'a self) -> i32 {
        3
    }
}
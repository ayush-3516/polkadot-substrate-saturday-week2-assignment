struct ImportantExcerpt<'a> {
    part: &'a str,
}
fn main() {
    println!("Success!")
}
impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
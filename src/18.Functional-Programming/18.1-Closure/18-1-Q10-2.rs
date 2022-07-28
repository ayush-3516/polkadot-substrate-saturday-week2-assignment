fn main() {
    let fn_plain = create_fn();
    fn_plain(1);
}
fn create_fn() -> Box<dyn Fn(i32) -> i32> {
    let num = 5;
    Box::new(move |x| x + num)
}

#[derive(Debug)]
struct NoCopyType {}
#[derive(Debug)]
#[allow(dead_code)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType
}
fn main()
{
    let no_copy = NoCopyType {};
    let example = Example { a: &1, b: &no_copy };
    fix_me(&example);
    print!("Success!")
}
fn fix_me<'b>(foo: &Example<'_, 'b>) -> &'b NoCopyType{ foo.b }
#[derive(Debug)]
struct NoCopyType {}
#[derive(Debug)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType
}
fn main()
{ 
  let var_a = 35;
  let example: Example;
  let var_b = NoCopyType {};
  example = Example { a: &var_a, b: &var_b };
  println!("(Success!) {:?}", example);
}
use std::fmt::Debug;
fn main() {
    const i:i32 = 5;
    print_it(i);
    print_it(&i);
    print_it1(&i);
    print_it2(&i);
}
fn print_it<T: Debug + 'static>( input: T) {
    println!( "'static value passed in is: {:?}", input );
}
fn print_it1( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}
fn print_it2<T: Debug + 'static>( input: &T) {
    println!( "'static value passed in is: {:?}", input );
}
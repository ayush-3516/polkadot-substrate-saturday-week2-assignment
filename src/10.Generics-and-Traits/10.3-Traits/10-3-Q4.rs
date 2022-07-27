use std::ops;
struct Foo;
struct Bar;
#[derive(PartialEq, Debug)]
struct FooBar;
#[derive(PartialEq, Debug)]
struct BarFoo;
fn main() {
    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Foo - Bar, BarFoo);
}
impl ops::Add<Bar> for Foo {
    type Output = FooBar;
    fn add(self, _rhs: Bar) -> FooBar {
        FooBar
    }
}
impl ops::Sub<Bar> for Foo {
    type Output = BarFoo;
    fn sub(self, _rhs: Bar) -> BarFoo {
        BarFoo
    }
}
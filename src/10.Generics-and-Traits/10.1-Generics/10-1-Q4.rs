fn main() {
    let p = Point{x: 5, y : "hello".to_string()};
}
struct Point<T, U> {
    x: T,
    y: U,
}
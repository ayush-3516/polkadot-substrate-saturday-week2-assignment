fn main() {
    let s = "Hello, world!";
    println!("{0:.5}", s);
    assert_eq!(format!("Hello {1:.0$}!", 3, "abcdefg"), "Hello abc!");
    println!("Success!")
}
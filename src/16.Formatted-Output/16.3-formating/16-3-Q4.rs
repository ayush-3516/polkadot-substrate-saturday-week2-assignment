fn main() {
    println!("Hello {:<5}!", "x");
    assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");
    assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !");
    assert_eq!(format!("Hello {:&<5}!", "x"), "Hello x&&&&!");
    println!("Success!")
}
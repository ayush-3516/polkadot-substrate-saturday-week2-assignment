fn main() {
    println!("Hello {:5}!", 5);
    println!("Hello {:+}!", 5);
    println!("Hello {:05}!", 5);
    println!("Hello {:05}!", -5);
    assert!(format!("{number:0>width$}", number=1, width=6) == "000001");
    println!("Success!")
}
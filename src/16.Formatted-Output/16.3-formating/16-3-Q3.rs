fn main() {
    println!("Hello {:5}!", "x");  
    println!("Hello {:1$}!", "x", 5);
    assert_eq!(format!("Hello {1:0$}!", 5, "x"), "Hello x    !");
    assert_eq!(format!("Hello {:width$}!", "x", width = 5), "Hello x    !");
    println!("Success!")
}
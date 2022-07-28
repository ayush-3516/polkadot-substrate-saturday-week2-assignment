fn main() {
    let static_string = "I'm in read-only memory";
    println!("static_string: {}", static_string);
    println!("static_string reference remains alive: {}", static_string);
}
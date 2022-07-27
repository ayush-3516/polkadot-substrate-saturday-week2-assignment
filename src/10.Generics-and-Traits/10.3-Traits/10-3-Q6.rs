struct Sheep {}
struct Cow {}
trait Animal {
    fn noise(&self) -> String;
}
fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}
impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaaaah!".to_string()
    }
}
impl Animal for Cow {
    fn noise(&self) -> String {
        "moooooo!".to_string()
    }
}
fn random_animal(random_number: f64) -> impl Animal {
    if random_number < 0.5 {
        Sheep {}
    } else {
        Sheep {}
    }
}
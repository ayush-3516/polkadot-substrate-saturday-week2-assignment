struct Duck;
struct Swan;
trait Bird {
    fn quack(&self) -> String;
}
fn main() {
    let duck = Duck;
    duck.swim();
    let bird = hatch_a_bird(2);
    assert_eq!(bird.quack(), "duck duck");
    let bird = hatch_a_bird(1);
    assert_eq!(bird.quack(), "swan swan");
    println!("Success!")
}   
fn hatch_a_bird(species: u8) ->Box<dyn Bird> {
    if species == 1 {
        Box::new(Swan{})
    } else {
        Box::new(Duck{})
    }
}
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}
impl Bird for Duck {
    fn quack(&self) -> String{
        "duck duck".to_string()
    }
}
impl Bird for Swan {
    fn quack(&self) -> String{
        "swan swan".to_string()
    }
}
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}
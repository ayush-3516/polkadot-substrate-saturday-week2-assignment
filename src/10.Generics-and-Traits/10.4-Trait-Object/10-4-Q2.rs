struct Duck;
struct Swan;
trait Bird {
    fn quack(&self);
}
fn main() {
    let birds: [Box<dyn Bird>; 2] = [Box::new(Duck {}), Box::new(Swan {})];
    for bird in birds {
        bird.quack();
    }
}
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}
impl Bird for Duck {
    fn quack(&self) {
        println!("{}", "duck duck");
    }
}
impl Bird for Swan {
    fn quack(&self) {
        println!("{}", "swan swan");
    }
}
impl Duck {
    fn fly(&self) {
        println!("Look, the duck is flying")
    }
}
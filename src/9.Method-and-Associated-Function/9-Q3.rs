fn main() {}
struct TrafficLight {
    color: String,
}
impl TrafficLight {
    pub fn show_state(self: &Self) {
        println!("the current state is {}", self.color);
    }
    pub fn change_state(&mut self) {
        self.color = "green".to_string()
    }
}
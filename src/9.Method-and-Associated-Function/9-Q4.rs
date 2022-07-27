#[derive(Debug)]
struct TrafficLight {
    color: String,
}
fn main() {
    let light = TrafficLight::new();
    assert_eq!(light.get_state(), "red");
}
impl TrafficLight {
    pub fn new() -> Self {
        Self {
            color: "red".to_string()
        }
    }
    pub fn get_state(&self) -> &str {
        &self.color
    }
}
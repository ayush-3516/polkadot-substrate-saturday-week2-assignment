fn main() {
    let c = TrafficLightColor::Yellow;
    assert_eq!(c.color(), "yellow");
    println!("{:?}", c);
}
#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}
impl TrafficLightColor {
    fn color(&self) -> String {
        match *self {
            TrafficLightColor::Red => "red".to_string(),
            TrafficLightColor::Yellow => "yellow".to_string(),
            TrafficLightColor::Green => "green".to_string(),
        }
    }
}
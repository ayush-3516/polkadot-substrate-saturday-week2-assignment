struct Interface<'b, 'a: 'b> {
    manager: &'b mut Manager<'a>
}
struct Manager<'a> {
    text: &'a str
}
struct List<'a> {
    manager: Manager<'a>,
}
fn main() {
    let mut list = List {
        manager: Manager {
            text: "hello"
        }
    };
    list.get_interface().noop();
    println!("Interface should be dropped here and the borrow released");
    use_list(&list);
}
impl<'a> List<'a> {
    pub fn get_interface<'b>(&'b mut self) -> Interface<'b, 'a>
    where 'a: 'b {
        Interface {
            manager: &mut self.manager
        }
    }
}
impl<'b, 'a: 'b> Interface<'b, 'a> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}
fn use_list(list: &List) {
    println!("{}", list.manager.text);
}
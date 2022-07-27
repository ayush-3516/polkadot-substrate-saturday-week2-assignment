trait Summary {
    fn summarize(&self) -> String;
}
#[derive(Debug)]
struct Post {
    title: String,
    author: String,
    content: String,
}
#[derive(Debug)]
struct Weibo {
    username: String,
    content: String,
}
fn main() {
    let post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweet".to_string(),
    };
    summary(&post);
    summary(&weibo);
    println!("{:?}", post);
    println!("{:?}", weibo);
}
fn summary(t: &impl Summary) {
    let _ = t.summarize();
}
impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo {}", self.username, self.content)
    }
}
impl Summary for Post {
    fn summarize(&self) -> String {
        format!("The author of post {} is {}", self.title, self.author)
    }
}
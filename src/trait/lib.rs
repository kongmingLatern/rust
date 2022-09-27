use std::fmt::Display;
pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location);
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {})", self.username, self.content);
    }
}
// trait bound 语法，可用于复杂情况
pub fn notify(item: impl Summary + Display) {
    println!("Breaking news! {}", item.summarize());
}
// 泛型
pub fn notify_1<T: Summary + Display>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notify_2<T, U>(a: T, b: U) -> String where T: Summary + Display, U: Clone + Debug {
    format!("{} {}", a.summarize(), b.summarize())
}
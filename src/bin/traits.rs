use std::fmt::{Debug, Display};

fn main() {}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

trait Summarize {
    fn summary(&self) -> String;
    fn default_msg(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summarize for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summarize for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn tweet_summ() {
    let t = Tweet {
        username: String::from("horse_ebook"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", t.summary())
}

pub fn notify<T: Summarize + Display>(item: T) {
    println!("Breaking news: {}", item.summary())
}
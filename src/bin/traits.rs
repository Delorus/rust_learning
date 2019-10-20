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

pub trait Summarize {
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

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

trait Draw {
    fn draw(&self);
}

struct Screen {
    components: Vec<Box<dyn Draw>>
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        unimplemented!()
    }
}

fn use_trait_object() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 75,
                height: 10,
                label: String::from("Ok"),
            })
        ]
    };

    screen.run();
}
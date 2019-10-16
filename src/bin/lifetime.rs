use std::fmt::Display;

fn main() {
    mutable_ref()
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn mutable_ref() {
    let x = &mut 10;
    plus(x, &1);
    println!("{}", x)
}

fn plus<'a, 'b>(x: &'a mut i32, y: &'b i32) -> &'a i32 {
    *x += *y;
    x
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn lifetime_in_struct() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence
    };

    println!("{}", i.part);
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display {
    println!("Anons: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
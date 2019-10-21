fn main() {
    destruct_match();
}

fn or_match() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two!"),
        3 => println!("three"),
        _ => println!("other"),
    };
}

fn range_match() {
    let x = 5;

    match x {
        1...5 => println!("one through five"),
        _ => println!("other"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn destruct() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    println!("a = {}", a);
    println!("b = {}", b);

    let Point { x, y } = p;
    println!("x = {}", x);
    println!("y = {}", y);
}

fn destruct_match() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn destruct_enum() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r, g, b
            )
        }
    }
}

fn ignore_several_value() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }
}

fn match_guard() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

fn binding_in_match() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_variable @ 3...7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10...12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
}
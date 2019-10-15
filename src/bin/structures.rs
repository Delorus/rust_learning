fn main() {
    empty_struct()
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn create_user() {
    let mut user = User {
        username: String::from("example"),
        email: String::from("example@example.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("user email: {}", user.email);

    user.email = String::from("new_email@example.com");

    println!("user email: {}", user.email);
}

fn create_user_from_another() {
    let user = User {
        username: String::from("example"),
        email: String::from("example@example.com"),
        sign_in_count: 1,
        active: true,
    };

    let another_user = User {
        username: String::from("new user"),
        ..user
    };

    println!("{:?}", another_user);
}

struct Point(i32, i32);

fn named_tuple() {
    let p = Point(1, 2);

    println!("x = {}", p.0);
}

#[derive(Debug)]
struct Empty;

fn empty_struct() {
    let empty = Empty;

    println!("{:?}", empty)
}


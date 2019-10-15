fn main() {
    return_ownership()
}

fn change_string() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s)
}

fn move_string_borrowed() {
    let s1 = String::from("hello");
    let s2 = s1/*.copy()*/;

//    println!("s1 = {}", s1); // not valid
    println!("s2 = {}", s2);
}

fn move_ownership() {
    let s = String::from("hello");

    takes_ownership(s);

//    let x = s.len(); // not valid
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn return_ownership() {
    let s1 = gives_ownership();

    let s1 = takes_and_gives_back_ownership(s1);
    println!("{}", s1);

    fn gives_ownership() -> String {
        let s = String::from("hello");
        println!("{}", s);
        return s;
    }

    fn takes_and_gives_back_ownership(s: String) -> String {
        println!("{}", s);
        return s;
    }
}

fn return_ownership_via_ref() {
    let s = String::from("hello");

    not_take_ownership(&s);
    println!("{}", s);

    fn not_take_ownership(s: &String) {
        println!("{}", s)
    }
}

fn ref_and_ownership() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("{}", s);

    fn change(s: &mut String) {
        s.push_str(", world!");
    }
}


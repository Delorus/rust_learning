use std::{io, result};

fn main() {
    synonym_type();
}

type Kilometers = i32;

fn synonym_type() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

type Thunk = Box<dyn Fn() + Send + 'static>;

type Result = result::Result<T, io::Error>;

fn never_return() -> ! {

}


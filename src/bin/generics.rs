use std::fmt::{Display, Debug};

fn main() {
    get_special_realisation();

}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn pow_x(&self) -> f32 {
        &self.x * 2f32
    }
}

fn create_generic_point() {
    let int = Point {
        x: 5,
        y: 5,
    };
    let float = Point {
        x: 1.0,
        y: 4.0,
    };
}

fn get_special_realisation() {
    let p = Point {
        x: 2.0,
        y: 2.0,
    };

    println!("{}", p.x);
    println!("{}", p.pow_x());
}

fn some_func<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug {

}
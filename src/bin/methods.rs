fn main() {
    print_area()
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            length: size,
            width: size,
        }
    }

    fn area(&self) -> u32 {
        self.length * self.width
    }
}

fn print_area() {
    let rect = Rectangle {
        length: 50,
        width: 30,
    };

    println!("area = {}", rect.area())
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length
            && self.width > other.width
    }
}


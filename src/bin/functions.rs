fn main() {
    fn_with_args(1, 2);
}

fn fn_with_args(x: i32, y: i32) {
    println!("x = {}, y = {}", x, y);
}

fn return_value() -> i32 {
    42
}

fn not_return_value() -> i32 {
    42; 66
}
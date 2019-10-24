fn main() {

}

fn return_closures() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
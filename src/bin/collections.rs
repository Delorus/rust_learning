fn main() {
    get_value()
}

fn create_vector() {
    let mut v = Vec::new();
    v.push(1);
    println!("{:?}", v);

    let v = vec![1, 2, 3];
    println!("{:?}", v);
}

fn get_value() {
    let mut v = vec![1, 2, 3, 4, 5];

    let third = v[2];
    println!("el = {}", third);
    println!("{:?}", v);
}
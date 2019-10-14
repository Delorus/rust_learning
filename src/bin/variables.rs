fn main() {
    shadow_immutable_var();
}

fn mutable_var() {
    let mut x = 5;
    println!("x: {}", x);

    x = 6;
    println!("x: {}", x);
}

fn constant() {
    println!("MAX_POINTS is: {}", MAX_POINTS);

    const MAX_POINTS: u32 = 100_000;
    const_in_func();
}

fn const_in_func() {
    println!("MAX_POINTS in func: {}", MAX_POINTS);
}

const MAX_POINTS: u32 = 200_000;

fn multiple_vars() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("x: {}", x)
}

fn shadow_immutable_var() {
    let spaces = "    ";
    let spaces = spaces.len();
    println!("Spaces: {}", spaces)
}
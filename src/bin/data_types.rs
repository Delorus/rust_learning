fn main() {
    std_f32()
}

fn cast_to_unsigned() {
    let guess: u32 = "-42".parse().expect("Not a number");
    println!("guess = {}", guess)
}

fn representation_of_number() {
    let value = 98_222;
    println!("value = {}", value);

    let value = 0xff;
    println!("value = {}", value);

    let value = 0o77;
    println!("value = {}", value);

    let value = 0b1111_0000;
    println!("value = {}", value);

    let value = b'A';
    println!("value = {}", value);
}

fn implicit_cast() {
    let x = 42i32;
    let y: u32 = 10;

    let z = x as u32 + y;
}

fn std_f32() {
    println!("DIGITS = {}", std::f32::DIGITS);

    println!("EPSILON = {}", std::f32::EPSILON);

    println!("INFINITY = {}", std::f32::INFINITY);

    println!("MANTISSA DIGIT = {}", std::f32::MANTISSA_DIGITS);

    // Maximum possible power of 10 exponent.
    println!("MAX_10_EXP = {}", std::f32::MAX_10_EXP);

    // Maximum possible power of 2 exponent.
    println!("MAX_EXP = {}", std::f32::MAX_EXP);

    // Smallest finite f32 value.
    println!("MIN = {}", std::f32::MIN);

    // Minimum possible normal power of 10 exponent.
    println!("MIN_10_EXP = {}", std::f32::MIN_10_EXP);

    // One greater than the minimum possible normal power of 2 exponent.
    println!("MIN_EXP = {}", std::f32::MIN_EXP);

    // Smallest positive normal f32 value.
    println!("MIN_POSITIVE = {}", std::f32::MIN_POSITIVE);

    // Not a Number (NaN).
    println!("NAN = {}", std::f32::NAN);

    // Negative infinity (-∞).
    println!("NEG_INFINITY = {}", std::f32::NEG_INFINITY);

    // The radix or base of the internal representation of f32.
    println!("RADIX = {}", std::f32::RADIX);
}
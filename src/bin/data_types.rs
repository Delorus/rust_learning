fn main() {
    arrays()
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

fn logic_op() {
    let t = true;

    println!("t = {}", t);
    println!("!t = {}", !t);
    println!("t && t = {}", t && t);
    println!("t || t = {}", t || t);

    let f = false;

    println!("f = {}", f);
    println!("!f = {}", !f);
    println!("f && f = {}", f && f);
    println!("f || f = {}", f || f);
}

fn chars() {
    let c = 'z';
    let z = 'ℤ';
    let smile = '😻';

    println!("c = {}", c);
    println!("z = {}", z);
    println!("heart_eyed_cat = {}", smile);
}

fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup = {:?}", tup);

    let (x, y, z) = tup;
    println!("The value of (x,y,z) is: ({},{},{})", x, y, z);

    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("The value of (x,y,z) is: ({},{},{})", x, y, z);
}

fn arrays() {
    let a = [1,2,3,4,5];
    let b: [u8; 5] = [1,2,3,4,5];
    println!("a is: {:?}", a);
    println!("b is: {:?}", b);

    let non_static_size: [u8; get_size()];
    non_static_size = [1, 2];
    println!("a is: {:?}", non_static_size);
}

const fn get_size() -> usize {
    1 + 1
}
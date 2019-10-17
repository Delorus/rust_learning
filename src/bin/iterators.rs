fn main() {
    using_counter_iter();
}

fn iter_terminated() {
    let v = [1, 2, 3];

    let v_iter = v.iter();

    let total: i32 = v_iter.sum();
}

fn iter_adaptors() {
    let v = [1, 2, 3];

    let v2: Vec<_> = v.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        return if self.count <= 5 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn using_counter_iter() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    println!("sum = {}", sum);
}
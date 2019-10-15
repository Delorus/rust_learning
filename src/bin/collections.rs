use std::collections::HashMap;

fn main() {
    get_pig_latin();
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

fn create_hash_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);
    println!("{:?}", scores);
}

fn get_stats() {
    let sts = stats(&vec![1, 2, 3, 3]);
    println!("avg = {}", sts.0);
    println!("median = {}", sts.1);
    println!("frequency usage = {}", sts.2);
}

fn stats(v: &Vec<i32>) -> (f64, i32, i32) {
    let avg = {
        let mut sum = 0;
        for i in v.iter() {
            sum += *i;
        }
        sum as f64 / v.len() as f64
    };

    let median = if v.len() == 0 { 0 } else {
        let middle = v.len() / 2;
        v[middle]
    };

    let freq_value = {
        let mut key = &0;
        let mut last_max = 0;
        let mut map = HashMap::new();
        for i in v.iter() {
            let count = map.entry(*i).or_insert(0);
            *count += 1;

            if *count > last_max {
                last_max = *count;
                key = i;
            }
        }

        *key
    };

    return (avg, median, freq_value);
}

fn get_pig_latin() {
    match convert_to_pig_latin(String::from("first"))  {
        Err(e) => println!("Error: {}", e),
        Ok(s) => println!("Pig latin: {}", s)
    }
}

fn convert_to_pig_latin(mut s: String) -> Result<String, String> {
    if s.trim().is_empty() {
        return Err("String must not be empty".to_string());
    }

    const VOWELS: [char;6] = ['a','e','i','o','u','y'];
    const CONSON: [char;18] = ['b', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'x', 'z'];

    let c = s.to_lowercase().chars().next().unwrap();
    if CONSON.contains(&c) {
        let s: String = s.drain(1..).collect();
        Ok(format!("{}-{}ay", s, c))
    } else if VOWELS.contains(&c) {
        Ok(format!("{}-hay", s))
    } else {
        Err("First char must be in latin".to_string())
    }
}
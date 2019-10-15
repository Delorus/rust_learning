fn main() {
    get_first_word();
}

fn get_first_word() {
    let s = "hello, man!";
    let word = first_word(s);

    println!("first word in str: [{}] is: {}", s, word);

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            };
        }

        &s[..]
    }
}


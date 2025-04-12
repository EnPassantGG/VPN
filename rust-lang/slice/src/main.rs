fn main() {
    let word = String::from("Spacing 7");
    let len = first_word(&word);
    println!("Word: {}, Function: {}", word, len);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
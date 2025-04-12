fn main() {
    let word = String::from("Spacing 7");
    let len = first_word(&word);
    println!("Word: {}, Function: {}", word, len);
}

// We can be more efficient by returning a slice, thereby 'binding'
// the String to the usize variable so that they're always together
fn first_word(s: &String) -> usize {
    // Check the String char by char
    let bytes = s.as_bytes();

    // Create an iterator over the array
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // byte literal syntax for space
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // now we don't need to keep track of the
                            // index and original String
        }
    }

    &s[..]
}
fn main() {
    // String slices
    let word = String::from("Spacing 7");
    let len = first_word_slice(&word);
    println!("Word: {}, Function: {}", word, len);

    // Other slices (integer)
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
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

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // now we don't need to keep track of the
                            // index and original String
        }
    }

    &s[..]
}
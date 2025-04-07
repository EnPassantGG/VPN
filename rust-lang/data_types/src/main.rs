fn parse_expect(value: &str) -> u32 {
    return value.parse().expect("Not a number!")
}

fn parse_expect_print(guess: u32) {
    println!("The value of _guess is: {guess}");
}

fn main() {
    // Loop through array and convert strings to ints
    let arr: [&str; 3] = ["31", "144", "541"];
    for guess in arr.iter() {
        let _guess = parse_expect(guess);
        parse_expect_print(_guess);
    }

    // Print out the array but by using the array indices
    let num_arr = [13, 7, 81];
    for i in 0..num_arr.len() {
        println!("Element {}: {}", i, num_arr[i]);
    }
}

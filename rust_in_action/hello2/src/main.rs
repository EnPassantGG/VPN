// Displays "Hello World" in three different languages
// Author: Jacob Hall
// Date: 03/30/2025
//
// Demonstrates Rust's built-in support for Unicode, allowing multiple languages
// to coexist. Also showcases Rust's easy iteration of elements

fn greet_world() {
    println!("Hello, world!");
    // Declares a block of code for a length of chars
    // Strings = UTF-8 size
    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";

    // Declares a block of code for two lengths of chars back to back
    let regions = [southern_germany, japan];

    // Goes through each region by starting at the beginning point of the first char
    for region in regions.iter() {
        // Fills the print in with the information starting at region
        println!("{}", &region); // & = read only
    }
}

fn main() {
    greet_world();
}

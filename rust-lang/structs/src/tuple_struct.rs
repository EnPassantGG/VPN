// These are good when naming something would be
// verbose or unnecessary
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

pub fn tuple_struct() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Print the values of the tuple struct
    println!("Color: {} {} {}", black.0, black.1, black.2);
}
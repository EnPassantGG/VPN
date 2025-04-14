// Named variables make everything obvious
struct Rectangle {
    width: u32,
    height: u32,
}

// Using structs as parameters make it obvious that the
// data is tied together (the width and height)
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


pub fn example_program() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
    );
}
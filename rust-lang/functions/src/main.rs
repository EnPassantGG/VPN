fn print_u32(x: u32) {
    println!("{x} must be positive!")
}

fn print_i32(x: i32) {
    println!("{x} must be negative!")
}

fn print_area_of_rectangle(length: u32, width: u32, height: u32) {
    let area = length * width * height;
    println!("Area: {area} units cubed");
}

fn main() {
    let positive_x: u32 = 5;
    let negative_x: i32 = -7;

    print_u32(positive_x);
    print_i32(negative_x);

    print_area_of_rectangle(3, 4, 10);
}
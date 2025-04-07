fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const HOURS_IN_A_YEAR: i32 = 24 * 365;
    println!("Hours in a year: {HOURS_IN_A_YEAR}");

    const PIE: f32 = 3.14;
    let radius = 3f32;
    let surface_area = PIE * radius * radius;
    println!("Surface area of circle with r=3: {surface_area}");
}

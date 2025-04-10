fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 9.0/5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32f32) * 5.0/9.0
}

fn main() {
    let fahrenheit = celsius_to_fahrenheit(84f32);
    println!("fahrenheit: {fahrenheit}");

    let celsius = fahrenheit_to_celsius(32f32);
    println!("celsius: {celsius}");

}

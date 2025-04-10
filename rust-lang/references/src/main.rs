mod immutable_reference;
mod dangling_references;

fn main() {
    let s1 = String::from("My name");

    // Because the reference only points to a value there is no ownership
    // taking place, so when &s1 leaves the scope it is fine because s1
    // is still in scope
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
    dangling_references::dangling_main();
}

// &: Refer to some value without taking ownership of it
fn calculate_length(s: &String) -> usize {
    s.len()
} // Don't need to return the String because we never had ownership
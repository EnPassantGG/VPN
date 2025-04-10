pub fn dangling_main() {
    let reference_to_nothing = dangle();
}

// This doesn't work because you are returning a reference
// to a variable whose memory has been deallocated.
// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s
// }

// Here, ownership is moved out, so nothing is deallocated
fn dangle() -> String {
    let s = String::from("hello");

    s
}
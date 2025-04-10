pub fn side_program() {
    let s = String::from("hello");
    takes_ownership(s); // s's value moves into the function
                        // and so is no longer valid here

    let x = 5;     // because i32 implements the Copy trait,
    makes_copy(x);      // x does not move into the function
    println!("That same integer again: {x}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
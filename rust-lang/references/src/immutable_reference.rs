pub fn immutable_main() {
    let mut s = String::from("hello");

    // Should use mutable references when you still want to use the
    // value that you pass into the function. If you don't use references
    // (and you don't return that same value) then you cannot use
    // that value inside the original scope.
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
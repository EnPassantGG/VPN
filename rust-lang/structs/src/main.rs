mod tuple_struct;
mod example;
mod debug;
mod methods;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    // Populate fields for a struct
    let mut user1 = User {
        active: true,
        username: String::from("myusername123"),
        email: String::from("myemail@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // Same as above, but with a method
    let email = String::from("example@gmail.com");
    let username = String::from("halolover");

    let user2 = build_user(email, username);

    // Copy values over from another struct but replace some values
    // However, we can no longer use user1 as a whole because String
    // username was moved over.
    let user3 = User {
        email: String::from("different@gmail.com"),
        ..user1
    };

    // Tuple structs
    tuple_struct::tuple_struct();

    // How to use structs
    example::example_program();

    // How to print debug statements
    debug::pretty_print();

    // Methods (use structs)
    methods::methods();
}

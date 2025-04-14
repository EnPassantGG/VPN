#[derive(Debug)]
struct Cube {
    length: u32,
    width: u32,
    height: u32,
}

pub fn pretty_print() {
    let cube1 = Cube {
        length: 10,
        width: 2,
        height: 4,
    };

    println!("cube1 is {cube1:?}")
}
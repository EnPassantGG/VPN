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

    println!("cube1 is {cube1:?}");

    let scale = 2;
    let cube2 = Cube {
        length: 10,
        width: dbg!(2 * scale),
        height: 4,
    };

    dbg!(&cube2);
}
// this makes rectangle able to debug its values
// add derive
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // :? will print the values
    println!("rect1 is {:?}", rect1);

    // :#? will prettify print
    // dbg! will debug the given value f.e.: dbg!(30 * something), dbg!(&struct)
}
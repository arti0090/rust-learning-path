fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    
    // not mutable, will fail
    // adding mut will fix it
    x = 6;
    println!("The value of x is: {x}");

    //constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}
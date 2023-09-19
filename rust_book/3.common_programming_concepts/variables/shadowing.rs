fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // shadowing lets create new variable with same name
    let spaces = "   ";
    let spaces = spaces.len();

    // but you cant mutate a variable type
    // let mut spaces = "   ";
    // spaces = spaces.len();
}
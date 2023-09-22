fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5]; // starting and ending index of given word
    // [0..5] == [..5]
    let world = &s[6..11];
    // [6..11] == [6..] 
}

// Function that returns first word from string or full word if no space
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // using slices
        }
    }

    &s[..]
}
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

// Slices as params

fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

// array slice

fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}
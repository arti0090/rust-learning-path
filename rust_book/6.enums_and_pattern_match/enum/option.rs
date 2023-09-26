// rust way to tuckle issue with null value
enum Option<T> {
    None,
    Some(T),
}

// option with int
let some_number = Some(5);
// option with char
let some_char = Some('e');

// type is Option<i32> but holds None (in some sense it is null)
let absent_number: Option<i32> = None;

// why to use it?
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
// in this example compiler will throw error that it does not know how to add
// Option<i8> + i8 - but we dont need to check if it is null, and error will be
// when we compile, not run app

// "In other words, you have to convert an Option<T> to a T
// before you can perform T operations with it.
// Generally, this helps catch one of the most common issues
// with null: assuming that something isn’t null when it actually is."
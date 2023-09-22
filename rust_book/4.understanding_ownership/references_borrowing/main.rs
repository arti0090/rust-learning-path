fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

// reference rather than ownership
fn calculate_length(s: &String) -> usize {  // s is a reference to a String
    s.len()
    s.push_str(", world"); // this will not work as s is only referenced
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // instantiate struct
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // get value
    user1.email = String::from("anotheremail@example.com");

    // "copying" user1 with new email as user2
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("antoher@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // shorthand version of copy
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let mut build_user = build_user("new@example.com", "new");
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username, // == username, -> shorthand
        email: email, // == email, => shorthand
        sign_in_count: 1,
    }
}
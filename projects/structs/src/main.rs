struct User {
    active: bool,
    username: String,
    email: String,
}

// Tuple Structs - they are a way of giving a na&me to a Tuple, without names fields though

struct Colour(i32, i32, i32);

// Unit Like Structs
// You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to ()
struct AlwaysEqual;

fn main() {
    let user = User {
        active: false,
        username: String::from("user_one"),
        email: String::from("user@email.com"),
    };

    println!("User {} is {}", user.username, get_active_term(user.active));

    let user = create_new_user("user_two");

    println!(
        "User {} is {}, reach them at {}",
        user.username,
        get_active_term(user.active),
        user.email
    );

    let black = Colour(0, 0, 0);
    println!("Black is {},{},{}", black.0, black.1, black.2);
}

fn get_active_term(active: bool) -> String {
    if active == true {
        return String::from("active");
    }
    return String::from("inactive");
}

fn create_new_user(username: &str) -> User {
    User {
        active: true,
        username: username.to_string(),
        email: String::from("user@email.com"),
    }
}

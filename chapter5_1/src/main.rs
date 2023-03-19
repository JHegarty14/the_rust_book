struct User {
    active: bool,
    // use owned String type instead of &str string slice type so that each
    // instance of the struct will own its data for as long as it is valid
    username: String,
    email: String,
    sign_in_count: u64,
}

// using tuple structs w/o fields to identify different types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs w/o fields
// can be useful for testing mocks
struct AlwaysEqual;

fn main() {
    let user = User {
        active: true,
        username: String::from("jhegarty14"),
        email: String::from("jhegarty14@gmail.com"),
        sign_in_count: 1,
    };

    let mut mut_user = User {
        active: true,
        username: String::from("jhegarty15"),
        email: String::from("jhegarty15@gmail.com"),
        sign_in_count: 1,
    };

    mut_user.email = String::from("newemail@gmail.com");

    let new_user = build_user(
        String::from("email@test.com"), 
        String::from("username"),
    );

    let new_user_2 = User {
        email: String::from("newuser@test.com"),
        ..new_user
    };

    // new_user is not usable as a whole bc we moved new_user.username
    // to new_user_2. If we had only moved active and sign_in_count, which both
    // implement the Copy trait, new_user would still be valid

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}

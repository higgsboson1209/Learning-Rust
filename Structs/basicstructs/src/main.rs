struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername2"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("someone2@example.com"),
        username: String::from("someusername2"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("someone3@example.com"),
        username: String::from("someusername3"),
        //This is called the struct update syntax 
        ..user1
    };
}
//building a function to create instances
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count:1
    }
}

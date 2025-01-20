// Structs
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    {
        // Immutable struct
        let user1 = User{
            active: true,
            username: String::from("Username"),
            email: String::from("Username@dot.com"),
            sign_in_count: 1,
        };
        
        let mut user2 = User{
            active: true,
            username: String::from("Username"),
            email: String::from("Username@dot.com"),
            sign_in_count: 1,
        };

        // Updating one field, but whole struct should be mut
        user2.email =String::from("Username@dot.net");

        // From other user
        let user3 = User {
            active: user1.active,
            username: user1.username,
            email: user1.email,
            sign_in_count: 2
        };

        // With better syntax
        let user4 = User {
            sign_in_count: 2,
            ..user2
        };
    }

    // tuple structs
    {
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }

    // Unit-like structs
    {
        struct AlwaysEqual;

        // Will be used later for traits
        let subject = AlwaysEqual;
    }
}

// Builder function with base params
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// and with shorter way
fn build_user_better(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
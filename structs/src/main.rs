struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main() {
    let some_data=User{
        active:true,
        username:String::from("someusername123"),
        email:String::from("someone@email"),
        sign_in_count:12,
    };

    println!("Username: {}", some_data.username);

    let user2=build_user(
        String::from("anotheremail@email"),
        String::from("anotherusername567"),
    );

    println!("Username: {}", user2.username);

    
    
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}


use std::io;

pub struct User {
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub active: bool,
}

pub fn build_user() -> User {
    let mut username = String::new();
    let mut email = String::new();
    let mut sign_in_count = 0;
    let mut active = false;

    println!("Enter username:");
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");

    println!("Enter email:");
    io::stdin()
        .read_line(&mut email)
        .expect("Failed to read line");

    User {
        username,
        email,
        sign_in_count,
        active,
    }
}
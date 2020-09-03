struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {

    let user = User {
        email: String::from("oooh@heit.net"),
        username: String::from("nerf"),
        sign_in_count: 12,
        active: true
    };

    println!("Hello, {}", user.username);

    let s1 = String::from("werwr@werwer");
    let s2 = String::from("qweqwe");

    let user2 = gimme_user(&s1, &s2);
    
}

fn gimme_user(email: &str, username: &str) -> User {

    User {
        email: String::from(email),
        username: String::from(username),
        active: true,
        sign_in_count: 1
    }

}


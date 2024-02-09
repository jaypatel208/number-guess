fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("thegreatkhali"),
        email: String::from("theycallmedaddy@gmail.com"),
        sign_in_count: 1,
    };

    println!(
        "Username:{}, E-mail:{}, Sign-in count:{}, Active user: {}",
        user1.username, user1.email, user1.sign_in_count, user1.active
    );

    user1.email = String::from("contactkhali@gmail.com");
    println!("New username: {}", user1.email);

    let email2 = String::from("simon@shadow.com");
    let username2 = String::from("ghost");

    let user2 = build_user(email2, username2);
    println!(
        "Ghost's email:{} and username:{}",
        user2.email, user2.username
    );

    let user3 = User {
        active: user1.active,
        username: String::from("alex"),
        email: user2.email,
        sign_in_count: user1.sign_in_count,
    };

    println!("Getting Alex's details.....");
    println!(
        "Alex: isActive:{}, username:{}, email:{},sign_in_count:{}",
        user3.active, user3.username, user3.email, user3.sign_in_count
    );

    let shadow = User {
        email: String::from("tf41force@gmail.com"),
        ..user1
    };

    println!("Get shadow's email: {}", shadow.email);

    // Unit-Like Structs Without Any Fields
    let subject = SeeYou;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct SeeYou;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

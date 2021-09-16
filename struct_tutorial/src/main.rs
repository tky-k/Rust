struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32,i32,i32);
struct Point(i32,i32,i32);
fn main() {
    println!("Hello, world!");
    let i = 32;
    let user1 = User {
        email: String::from("hoge@email.com"),
        username: String::from("hoge"),
        active: true,
        sign_in_count: 1,
    };

    println!("user email is {}", user1.email);

    let user = String::from("user_name");
    let mail = String::from("user_name@mail.com");
    let user2 = build_user(user, mail);
    println!("user is {}, mail is {}", user2.username, user2.email);

    let user3 = User {
        ..user2
    };
    println!("user3 is {}, mail is {}", user3.username, user3.email);
    println!("user2 active is  {}, user2 sign_in_count is {}", user2.active, user2.sign_in_count);

    let c = Color(1,2,3);
    param_Color(c);

}


fn build_user(username: String, email: String) -> User {
    User { username: (username), email: (email), sign_in_count: (1), active: (true) }
}
fn param_Color(c: Color) {

}

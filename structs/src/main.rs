#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someoneusername123"),
        sign_in_count: 1,
        active: true
    };

    println!("{}", user1.username);

    let mut user2 = user1;
    user2.email = String::from("anotheremail@example.com"); // mut する必要あり
    println!("{}", user2.email);

    let user3 = build_user(String::from("smngs@smngs.io"), String::from("smngs"));
    println!("{:#?}", user3);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // 名前付きフィールドのないタプル構造体
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);


    println!("{:?}", black);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true
    }
}

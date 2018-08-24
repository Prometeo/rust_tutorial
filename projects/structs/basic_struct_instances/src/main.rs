struct User {
    username: String,
    email: String,
    sing_in_count: u64,
    active: bool,
}

let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active:true,
    sing_in_count: 1,
};

let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sing_in_count: user1.sing_in_count,
}

let user3 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};


fn main() {
    println!("Hello, world!");
}

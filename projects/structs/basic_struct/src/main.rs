# struct User {
#    username: String,
#    email: String,
#    sign_in_count: u64,
#    active: bool,
# }
#

let user1 = User{
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
}

fn build_user(email:String, username:String) -> User
{
    User{
        email:email,
        username:username,
        active:true,
        sign_in_count:1,
    }
}

fn build_user(email:String, username:String) -> User
{
    User{
        email,
        username,
        active:true,
        sign_in_count:1,
    }
}

fn main() {



}

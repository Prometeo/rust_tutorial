enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> i32{
    match coin{
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Patterns that bind to values

#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> i32{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

let coin_1 = value_in_cents(Coin::Quarter(UsState::Alaska));

// Matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = pluse_one(None);

// The _ Placeholder
let some_u8_value = Ou8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (), // any other case
}
// end placeholder

// if let
let some_u8_value = Some(Ou8);
match some_u8_value {
    Some(3) => println!("three");
    _ => (),
}

if let Some(3) = some_u8_value{
    println!("three");
}

let coin = Coin::Penny;
let mut count = 0;
match coin{
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count +=1,
}

if let Coin::Quarter(state) = coin{
    println!("State quarter from {:?}!",state);
}else{
    count +=1;
}
// end if let

fn main() {
    println!("Hello, world!");
}

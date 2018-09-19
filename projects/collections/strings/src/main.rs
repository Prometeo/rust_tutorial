// create a new string
let mut s = String::new();

let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly
let s = "initial contents".to_string();

// this is equivalent to use to_string
let s = String::from("intial contents");

// strings are UTF-8 encoded so all these strings are valid
let hello = "   ";
let hello = "Dobrý den";
let hello = "hello";
let hello = "Olá";

// Updating a string
// Appending to a string with push
let mut s = String::from("foo");
s.push_str("bar");

// concatenation with + operator
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note that s1 has been moved here
// and can no longer be used

// concatenate multiple strings with "+"
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;

// concatenate wit format
let s = format!("{}-{}-{}", s1, s2, s3);

// Indexing into Strings
let s1 = String::from("hello");
let h = s1[0]; // this result into an error  `std::string::String` cannot be indexed by `_`

// Rust doesn't index strings, but disuades you to use slice
// with a range
let hello = "    ";
let s = &hello[0..4];

// &hello[0..1]? will give a panic at runtime

// Iterating over strings
// using chars
for c in "   ".chars() {
    println!("{}",c);
}

// using bytes
for b in "   ".bytes() {
    println!("{}",b);
}

fn main() {
    println!("Hello, world!");
}

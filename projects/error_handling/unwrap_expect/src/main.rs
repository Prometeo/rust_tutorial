use std::fs::File;

// unwrap will return the value inside Ok if result value is Ok variant
// unwrap will call panic! macro if the Result is the Err variant

fn main() {
    let f = File::open("hello.txt").unwrap();
}

// Expect

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn main() {
    // let mut s1 = String::from("hello");
    // let s2 = s1; // now s1 is invalid, only s2 is valid
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // types like integers that have a known size at compile time
    // are stored entirely on the stack, so copies of the actual values
    // are quick to make
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}

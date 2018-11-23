use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // Here we’re calling the hello function with the argument &m, which is
    // a reference to a MyBox<String> value. Because we implemented the Deref trait
    // on MyBox<T> in Listing 15-10, Rust can turn &MyBox<String> into &String by calling deref.
    // The standard library provides an implementation of Deref on String that returns a string slice,
    // and this is in the API documentation for Deref. Rust calls deref again to turn the
    // &String into &str, which matches the hello function’s definition.

    // If Rust didn’t implement deref coercion, we would have to write
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}

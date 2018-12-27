#![allow(unused_variables)]
fn main() {
    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }
}

// A trait is unsafe when at least one of its methods has some
// invariant that the compiler can’t verify

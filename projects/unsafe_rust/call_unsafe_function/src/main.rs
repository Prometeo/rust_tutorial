#![allow(unused_variables)]
fn main() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// demonstrates how to set up an integration with the abs function from
// the C standard library. Functions declared within extern blocks are
// always unsafe to call from Rust code. The reason is that other languages don’t
// enforce Rust’s rules and guarantees, and Rust can’t check them, so
// responsibility falls on the programmer to ensure safety.

// Calling Rust Functions from Other Languages
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// This usage of extern does not require unsafe.

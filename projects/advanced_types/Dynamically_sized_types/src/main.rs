// To work with DSTs, Rust has a particular trait called the Sized trait
// to determine whether or not a type’s size is known at compile time

fn generic<T>(t: T) {
    // --snip--
}

// is actually treated as though we had written this:
fn generic<T: Sized>(t: T) {
    // --snip--
}

// By default, generic functions will work only on types that have a known size at
// compile time. However, you can use the following special syntax to relax this restriction:
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}

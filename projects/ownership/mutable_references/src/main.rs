fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    // You can only have one mutable reference

    // This code will fail:
    // let r1 = &mut s;
    // let r2 = &mut s;
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

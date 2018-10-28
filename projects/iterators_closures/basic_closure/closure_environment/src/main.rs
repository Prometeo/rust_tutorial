fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    // with a function we wouldn't be able to call 'x'
    // fn equal_to_x(z: i32) -> bool { z == x }

    let y = 4;

    assert!(equal_to_x(y));
}

// closure taking ownership
fn main() {
    // using vectors instead of integers, because integers can be copied rather than moved
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // cant be used because equal_to_x has the ownership
    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

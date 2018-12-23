struct Point {
    x: i32,
    y: i32,
}

// This code creates the variables a and b that match
// the values of the x and y fields of the p struct
fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}

// let Point { x: x, y: y } = p; contains a lot of duplication,
// there is a shorthand for patterns that match struct fields
fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}

// We can also destructure with literal values as part of the struct
// pattern rather than creating variables for all the fields.
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

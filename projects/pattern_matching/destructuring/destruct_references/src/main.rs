#![allow(unused_variables)]
fn main() {
    struct Point {
        x: i32,
        y: i32,
    }

    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
}

// When the value we’re matching to our pattern contains a reference,
// we need to destructure the reference from the value, which we can do
// by specifying a & in the pattern.

// If we had not included the & in &Point { x, y }, we’d get a type mismatch error,
//because iter would then iterate over references to the items in the vector rather
// than the actual values.

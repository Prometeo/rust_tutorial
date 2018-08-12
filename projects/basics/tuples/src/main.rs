fn main() {
    let _tup: (i32, f64, u8) = (500, 6.4, 1); // optional
    // println!("optionaly type annotations: {}", tup);

    let tup2 = (500, 6.4, 1);
    let (_x, y, _z) = tup2;
    println!("The value of y is; {}", y);

    let x = tup2.0;
    println!("access by index: {}", x);
}

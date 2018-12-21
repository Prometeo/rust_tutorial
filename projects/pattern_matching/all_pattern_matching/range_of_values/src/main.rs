fn main() {
    // Ranges are only allowed with numeric values or char values,
    // because the compiler checks that the range isn’t empty at compile time.
    let x = 5;

    match x {
        1...5 => println!("one through five"),
        _ => println!("something else"),
    }

    // with char values
    let x = 'c';

    match x {
        'a'...'j' => println!("early ASCII letter"),
        'k'...'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

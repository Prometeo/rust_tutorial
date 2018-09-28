struct Point<T> {
    x: T,
    y: T,
}

struct PointB<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointB<T, U> {
    fn mixup<V, W>(self, other: PointB<V, W>) -> Point<T, W> {
        PointB {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // the next line wouldn't work, fields x and y must be the
    // same type because both have the same generic data type T
    let wont_work = Point { x: 5, y: 4.0 };

    // with mixup function
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

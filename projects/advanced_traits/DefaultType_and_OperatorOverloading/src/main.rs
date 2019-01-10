// When we use generic type parameters, we can specify a default concrete type for the generic type.
// This eliminates the need for implementors of the trait to specify a concrete type if the
// default type works. The syntax for specifying a default type for a generic type is <PlaceholderType=ConcreteType>

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

trait Add<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

// We have two structs, Millimeters and Meters, holding values in different units. We want to add values
// in millimeters to values in meters and have the implementation of Add do the conversion correctly.

use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

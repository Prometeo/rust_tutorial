// One example of a trait with an associated type is the Iterator trait that
// the standard library provides. The associated type is named Item and stands
// in for the type of the values the type implementing the Iterator trait is iterating over.

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

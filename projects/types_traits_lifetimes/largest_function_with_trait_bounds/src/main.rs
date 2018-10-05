use std::cmp::PartialOrd;

// types like i32 and char that have known size can be stored
// on the stack, so they implement Copy trait
// without the Copy trait we wouldn't be able to
// move the value out of list[0] and into the largest variable
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];

    let result = largest(&chars);
    println!("The largest char is {}", result);
}

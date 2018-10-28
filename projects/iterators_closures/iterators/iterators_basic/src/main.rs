fn main() {
    let v1 = vec![1, 2, 3];

    // This doesn do anything (iterators are lazy)
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // sum method takes wnership of the iterator and iterates
    // through the items by repeatedly calling next
    let total: i32 = v1_iter.sum();

    // We aren’t allowed to use v1_iter after the call to sum because
    // sum takes ownership of the iterator we call it on.
    assert_eq!(total, 6);
}

#[test]
fn iterator_map() {
    let v1: Vec<i32> = vec![1, 2, 3];

    // map takes a closure to call on each item to produce a new iterator.
    // the colect method consumes the iterator
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

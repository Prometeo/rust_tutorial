// To create a new empty vector:
let v: Vec<i32> = Vec::new();

// This ways is more common giving initial values and rust
// infer the type
let v = vec![1,2,3];

// Updating a Vector

let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
v.push(8);

// Accessing values of a vector
// method 1:
let third: &i32: &v[2];

// method 2
let third: Option<&i32> = v.get(2);

let does_not_exist = &v[100];
// will cause a panic when a non-existent element is referenced.
let does_not_exist = v.get(100);
// Will return None when a non-existent element is referenced.


// Invalid References
let mut v = vec![1,2,3,4,5];

let first = &v[0];
v.push(6);
// This would give error of borrowing rules(mutable - inmutable)
// Adding a new element onto the end of the vector might require allocating new
// memory and copying the old elements over to the new space, in the circumstance
// that there isn’t enough room to put all the elements  next to each other where
// the vector was. In that case, the reference to the first element would be pointing
// to deallocated memory.


// Using enumo to store multyple types
enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.2),
]

    fn main() {
        println!("Hello, world!");
    }

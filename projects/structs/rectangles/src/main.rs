fn main() {
    // let length1 = 50;
    // let width1 = 30;
    // let rect1 = (50,30);
    // struct with labeling data
    struct Rectangle {
        length: u32,
        width: u32,
    }

    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        // area(length1, width1)
        area(&rect1)
    );

    fn area(rectangle: &Rectangle) -> u32{
        rectangle.length * rectangle.width
    }
}

// fn area(length:u32, width:u32) -> u32 {
//     length * width
// }

// function area with tuple

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

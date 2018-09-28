fn largest(list: &[i32]) -> i32 {
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

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    // not generic way
    // let numbers = vec![34, 50, 25, 100, 65];
    // let mut largest = numbers[0];

    // for number in numbers {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    // println!("The largest number is {}",largest);
}

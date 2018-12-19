let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}

// In a for loop, the pattern is the value that directly follows
// the keyword for, so in for x in y the x is the pattern.

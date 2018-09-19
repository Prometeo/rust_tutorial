use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"),10);
scores.insert(String::from("Yellow"),50);

// creating a hashmap using collections
let teams = vec![String::from("Blue"),String::from("Yellow")];
let initial_scores = vec![10,50];
let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

// Ownership
let field_name = String::from("Favorite color");
let field_value = String::from("Blue");
let mut map = HashMap::new();
map.insert(field_name,field_value);
// field_name and file_value are invalid at this point

// Accessing Values in a hash map
let mut scores = HashMap::new();
scores.insert(String::from("Blue"),10);
scores.insert(String::from("Yellow"),50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);

// Iterate over each key/value
for (key,value) in &scores {
    println!("{}: {}",key,value);
}

// Updating a Hash Map

// overwriting a value
let mut scores = HashMap::new();
scores.insert(String::from("Blue"),10);
scores.insert(String::from("Blue"),50);

println!("{:?}",scores); // this should print {"Blue": 25}

// only insert if the key has no value
scores.insert(String::from("Blue"),10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

// Update a value based on the old value
let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count +=1;
    // n order to assign to that value we must first dereference
    // count using the asterisk (*)
}

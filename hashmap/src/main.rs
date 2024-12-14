use std::collections::{hash_map::Entry, HashMap};

fn main() {
    let mut hash_map = HashMap::new();

    hash_map.insert("name".to_string(), "John".to_string());

    hash_map.insert("age".to_string(), 30.to_string());

    println!("{:?}", hash_map);

    let name = hash_map.get("name");

    match name {
        Some(value) => println!("Name: {}", value),
        None => println!("Name not found"),
    }

    for (key, value) in &hash_map {
        println!("{}: {}", key, value);
    }

    hash_map.remove("name");

    let name = hash_map.get("name");

    match name {
        Some(value) => println!("Name: {}", value),
        None => println!("Name not found"),
    }

    hash_map.insert("age".to_string(), 31.to_string());

    println!("{hash_map:?}");

    hash_map
        .entry("name".to_string())
        .or_insert("John".to_string());

    println!("{hash_map:?}");

    let r = hash_map.entry("age".to_string());

    match r {
        Entry::Occupied(o) => {
            println!("Occupied: {:?}", o);
        }
        Entry::Vacant(v) => {
            println!("Vacant: {:?}", v);
            v.insert("30".to_string());
        }
    }

    println!("{hash_map:?}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    map.insert("hello", 1);
    map.insert("world", 2);

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}

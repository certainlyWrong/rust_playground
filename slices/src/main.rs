// create a struct person with constructor and method

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Person {
        Person {
            name: name.to_string(),
            age,
        }
    }
}

fn main() {
    let mut hello_world = String::from("hello world");

    let hello = first_world(&hello_world);

    println!("{}", hello);

    // let hello = &hello_world[0..5];
    // let world = &hello_world[6..11];

    // println!("{}", hello);
    // println!("{}", world);

    hello_world.clear();

    // println!("{}", hello); // error

    let hello_world = "hello world";

    let hello = first_world2(hello_world);

    println!("{}", hello);

    let hello_world = String::from("hello world");

    let hello = first_world2(&hello_world);

    println!("{}", hello);

    let person1 = Person::new("John", 30);
    let person2 = Person::new("Jane", 25);
    let person3 = Person::new("Doe", 20);
    let person4 = Person::new("Smith", 35);

    let persons = [person1, person2, person3, person4];

    for person in &persons[..2] {
        println!("Name: {}, Age: {}", person.name, person.age);
    }
}

// fn first_world(worlds: &String) -> usize {
//     let bytes = worlds.as_bytes();

//     for (i, &byte) in bytes.iter().enumerate() {
//         if byte == b' ' {
//             return i;
//         }
//     }

//     worlds.len()
// }

fn first_world(worlds: &String) -> &str {
    let bytes = worlds.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &worlds[0..i];
        }
    }

    &worlds[..]
}

fn first_world2(worlds: &str) -> &str {
    let bytes = worlds.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &worlds[0..i];
        }
    }

    &worlds[..]
}

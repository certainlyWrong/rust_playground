// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    // if x.len() > y.len() {
    //     x
    // } else {
    //     y
    // }
    println!("x: {x}, y: {y}");
    x
}

// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{}", i.part);
    println!("{:?}", i);
    println!("{}", i.level());

    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);

    // longest_with_an_announcement
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let ann = "Announcement!";

    let result = longest_with_an_announcement(string1.as_str(), string2.as_str(), ann);

    println!("The longest string is {result}");
}

// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {result}");
// }

// fn main() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {result}");
// }

// fn main() {
//     let r;

// {
//     let x = 5;
//     r = &x; // Error: `x` does not live long enough
// }

// println!("r: {r}");

//     {
//         let x = 5;
//         r = &x; // Error: `x` does not live long enough
//         println!("r: {r}");
//     }
// }

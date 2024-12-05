fn main() {
    // let a = "Hello";

    // println!("{}", a);
    // {
    //     let mut a = String::from(a);

    //     a.push_str(", World!");

    //     println!("{}", a);
    // }

    // println!("{}", a);

    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("{s2}, world!");
    // println!("{s1}, world!");

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s.clone()); // s's value moves into the function...
                                // ... and so is no longer valid here

    println!("{s}");

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward

    println!("{x}");

    let s1 = gives_ownership();

    println!("{s1}");

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("{s3}");
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

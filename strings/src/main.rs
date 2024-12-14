fn main() {
    let s = "hello world";

    let mut s = s.to_string();

    println!("{}", s);
    let s2 = "!";

    s.push_str(s2);
    println!("{}, {}", s, s2);

    let s3 = String::from("hello");

    println!("{}", s3);

    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = s1 + &s2;

    println!("{}", s3);
    println!("{}", s2);
    // println!("{}", s1); // error

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("{}", s);

    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("{}", s);
}

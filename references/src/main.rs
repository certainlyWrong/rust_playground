fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);

    {
        let r1 = &mut s;
        println!("{}", r1);
    }
    // let r2 = &mut s1; // error[E0499]: cannot borrow `s1` as mutable more than once at a time
    {
        let r2 = &mut s;
        println!("{}", r2);
    }

    let r3 = &s;
    let r4 = &s;
    let r5 = &mut s;
    println!("{}", r3);
    println!("{}", r4);
    println!("{}", r5);
}

fn change(s: &mut String) {
    s.push_str(", world");
}

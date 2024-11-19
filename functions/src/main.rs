fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = 6;

    let result = sum(x, y);

    println!("The result is: {}", result);

    let y = {
        let x = 3;
        sum(x, x)
    };

    println!("The value of y is: {}", y);

    println!("The value of five is: {}", five());

    println!("The value of plus_one(5) is: {}", plus_one(5));

    let f: fn(u32) -> u32 = plus_one;

    println!("The value of apply(f, 5) is: {}", apply(f, 5));
}

fn sum(x: u32, y: u32) -> u32 {
    x + y
}

fn five() -> u32 {
    5
}

fn plus_one(x: u32) -> u32 {
    x + 1
}

fn apply(f: fn(u32) -> u32, x: u32) -> u32 {
    f(x)
}

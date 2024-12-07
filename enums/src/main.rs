enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4(a, b, c, d) => println!("V4: {}.{}.{}.{}", a, b, c, d),
        IpAddrKind::V6(ip) => println!("V6: {}", ip),
    }
}

fn try_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Result::Err(String::from("Division by zero"))
    } else {
        Result::Ok(a / b)
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    route(four);

    route(loopback);

    let result = try_divide(10, 2);

    match result {
        Result::Ok(value) => println!("Result: {}", value),
        Result::Err(message) => println!("Error: {}", message),
    }

    let result = try_divide(10, 0);

    match result {
        Result::Ok(value) => println!("Result: {}", value),
        Result::Err(message) => println!("Error: {}", message),
    }

    let message = Message::Write(String::from("Hello, world!"));

    message.call();

    // Option methods
    let option = Some(5);

    if let Some(value) = option {
        println!("Value: {}", value);
    } else {
        println!("No value");
    }
}

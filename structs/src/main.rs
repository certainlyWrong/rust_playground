struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct PixelRGB {
    red: u8,
    green: u8,
    blue: u8,
}

struct Point(f32, f32, f32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };

    println!("User1 email: {}", user1.email);

    let mut user2 = User {
        email: String::from("someone2@example.com"),
        ..user1
    };

    println!("User2 email: {}", user2.email);

    user2.email = String::from("someone3@example.com");

    println!("User2 email: {}", user2.email);
    println!("User2 username: {}", user2.username);
    println!("User2 active: {}", user2.active);
    println!("User2 sign_in_count: {}", user2.sign_in_count);

    let black = PixelRGB {
        red: 0,
        green: 0,
        blue: 0,
    };

    println!(
        "Black pixel: R:{} G:{} B:{}",
        black.red, black.green, black.blue
    );

    let origin = Point(0.0, 0.0, 0.0);

    let Point(x, y, z) = origin;

    println!("Origin point: {} {} {}", x, y, z);

    println!("Origin point: {} {} {}", origin.0, origin.1, origin.2);

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (width1, height1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_rect(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_rect_struct(rect2)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");
    println!("rect1 is {rect1:#?}");

    dbg!(&rect1);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_rect(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

fn area_rect_struct(rect: Rectangle) -> u32 {
    rect.width * rect.height
}

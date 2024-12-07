#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Self { width, height }
    }

    fn square(size: u32) -> Rectangle {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, react: &Rectangle) -> bool {
        self.width > react.width && self.height > react.height
    }
}

fn main() {
    let rect1 = Rectangle::new(10, 20);

    let area = rect1.area();

    println!("The area of the rectangle is {} square pixels.", area);

    println!("The width of the rectangle is {}", rect1.width);

    println!("rect1 is {:?}", rect1);

    let rect2 = Rectangle::new(5, 10);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let square = Rectangle::square(10);

    println!("The square is {:?}", square);
}

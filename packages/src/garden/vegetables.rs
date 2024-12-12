#[derive(Debug)]
pub struct Asparagus {
    pub name: String,
    quantity: i32,
}

impl Asparagus {
    pub fn new(name: &str, quantity: i32) -> Asparagus {
        Asparagus {
            name: name.to_string(),
            quantity,
        }
    }

    pub fn grow(&self) {
        println!("Growing {} asparagus plants...", self.quantity);
    }
}

pub mod asparagus {
    pub fn plant() {
        println!("Planting asparagus in package...");
    }

    pub mod server {

        pub fn plant() {
            println!("Planting asparagus in rows...");
        }
    }

    pub mod client {
        pub fn plant() {
            println!("Planting asparagus in hills...");
        }
    }
}

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::garden::vegetables::front_of_house::hosting::add_to_waitlist();

    // self keyword
    self::front_of_house::hosting::add_to_waitlist();

    // super keyword
    super::vegetables::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

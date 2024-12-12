pub mod garden;
pub mod randon_numbers;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

mod customer {
    use super::front_of_house::hosting as server;

    pub fn eat_at_restaurant() {
        server::add_to_waitlist();
    }
}

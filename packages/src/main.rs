use packages::garden;

fn main() {
    let asparagus = garden::vegetables::Asparagus::new("Mary Washington", 15);
    println!("{:?}", asparagus);
    asparagus.grow();
    garden::asparagus::plant();
    garden::vegetables::asparagus::plant();
    garden::vegetables::asparagus::server::plant();
}

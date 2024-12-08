fn main() {
    let config_max = Some(10);

    if let Some(max) = config_max {
        println!("max: {}", max);
    } else {
        println!("max is None");
    }
}

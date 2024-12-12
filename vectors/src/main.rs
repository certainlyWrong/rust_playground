#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    v1.push(5);
    let v3 = [v1, v2].concat();

    println!("{:?}", v3);

    let third: &i32 = &v3[2];

    println!("The third element is {}", third);

    let third = v3.get(2);

    match third {
        Some(value) => println!("The third element is {}", value),
        None => println!("There is no third element."),
    }

    for i in &v3 {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    v.pop();

    for i in &v {
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);
}

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = String::from("hello");

    println!("The value of y is: {y}");

    {
        let y = y.len();

        println!("The value of y is: {y}");
    }

    println!("The value of y is: {y}");

    let z = aaa(5);

    println!("The value of z is: {z}");

    let z = aaa("hello");

    println!("The value of z is: {z}");

    let z = aaa(y);

    println!("The value of z is: {z}");

    let w = 57u8;

    println!("The value of w is: {w}");

    let w = 1_000;

    println!("The value of w is: {w}");

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("Sum: {sum}");
    println!("Difference: {difference}");
    println!("Product: {product}");
    println!("Quotient: {quotient}");
    println!("Remainder: {remainder}");

    let t = true;
    let f: bool = false;

    println!("The value of t is: {t}");
    println!("The value of f is: {f}");
    println!("Or operation: {}", t || f);
    println!("And operation: {}", t && f);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("The value of c is: {c}");
    println!("The value of z is: {z}");
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of x is: {z}");

    let tup = (500, 6.4, 1);

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("The value of first is: {first}");

    let a = [[1, 2], [3, 4], [5, 6]];
}

fn aaa<T>(x: T) -> T {
    x
}

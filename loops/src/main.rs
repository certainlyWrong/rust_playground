fn main() {
    // loop, while and for loop

    // loop
    let mut counter = 0;
    loop {
        counter += 1;
        println!("counter: {}", counter);
        if counter == 10 {
            break;
        }
    }

    let sum_values = loop {
        counter += 1;
        if counter == 20 {
            break counter * 2;
        }
    };

    println!("sumValues: {}", sum_values);

    // loops também podem usar return

    // Loop labels
    // uma maneira de sair de um loop aninhado
    // Pode-se defirnir qualquer nome para identificar as labels de cada loop
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }

        println!("This point will never be reached");
    }

    // while loop

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // while loop
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // for loop
    for number in a {
        println!("the value is: {}", number);
    }

    // reverse for loop
    for index in (0..=4).rev() {
        println!("{}!", a[index]);
    }
}

pub mod randon_numbers {
    // A linha seguinte é um exemplo de uso de um módulo aninhado
    // ela trará o módulo 'io' e 'Ordering' do módulo 'cmp' do módulo 'std'
    // use std::{cmp::Ordering, io};

    // A linha seguinte é um exemplo de uso de um módulo aninhado
    // ela trará o módulo 'io' e 'Write' do módulo 'io' do módulo 'std'
    use std::io::{self, Write};

    // A linha seguinte é um exemplo de uso do operador '*' (glob)
    // ela trará todos os itens do módulo 'io' do módulo 'std'
    // use std::io::*;

    pub fn guess_the_number() {
        println!("Guess the number!");

        let secret_number = 42;

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    }

    pub fn generate_random_number() -> i32 {
        42
    }
}

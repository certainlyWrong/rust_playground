use std::{
    env, fs,
    fs::File,
    io::{Error, ErrorKind, Read, Write},
};

fn read_username_from_file1() -> Result<String, Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// MyError
struct MyError {
    io: Error,
}

impl From<Error> for MyError {
    fn from(error: Error) -> Self {
        MyError { io: error }
    }
}

fn read_username_from_file2() -> Result<String, MyError> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file3() -> Result<String, Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    let hello_file: Vec<String> = env::args().collect();

    for arg in hello_file.iter() {
        println!("Argument: {}", arg);
    }

    let username = read_username_from_file2();

    // unwrap() will panic if the Result is an Err
    // File::open(hello_file[1].clone()).unwrap();

    // expect() will panic with the message provided if the Result is an Err
    // File::open(hello_file[1].clone()).expect("Failed to open file");

    // The File::open() function returns a Result<File> type or an Error the `?` operator can only be used in a function that returns `Result` or `Option`
    // let greeting_file = File::open("hello.txt")?;

    let greeting_file_result = File::open(hello_file[1].clone());

    match greeting_file_result {
        Ok(file) => {
            println!("File opened successfully: {:?}", file);

            let mut file_content = String::new();
            let read_result = file.take(10).read_to_string(&mut file_content);

            match read_result {
                Ok(_) => {
                    println!("File content: {}", file_content);
                }
                Err(error) => {
                    println!("Failed to read file: {:?}", error);
                }
            }
        }
        Err(error) => {
            println!("Failed to open file: {:?}", error);

            match error.kind() {
                ErrorKind::NotFound => {
                    println!("File not found");
                }
                ErrorKind::PermissionDenied => {
                    println!("Permission denied");
                }
                _ => {
                    println!("Unknown error");
                }
            }
        }
    }

    let greeting_file_result = File::open(hello_file[1].clone()).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            let mut file = File::create(hello_file[1].clone()).unwrap_or_else(|error| {
                panic!("Failed to create file: {:?}", error);
            });

            file.write(b"Hello, World!").unwrap_or_else(|error| {
                panic!("Failed to write to file: {:?}", error);
            });

            return file;
        } else {
            panic!("Failed to open file: {:?}", error);
        }
    });

    println!("File opened successfully: {:?}", greeting_file_result);
}

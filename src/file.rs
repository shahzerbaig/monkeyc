use std::env;
use std::io;
//use std::process;

pub fn read_file_name() -> Result<String, io::Error> {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(filename) => Ok(filename.clone()),
        None => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Please provide a Monkey source file",
        )),
    }
}

pub fn read_file_content(filename: String) -> Result<String, std::io::Error> {
    std::fs::read_to_string(filename)
}

pub fn print_file_content(filename: String) {
    match read_file_content(filename) {
        Ok(content) => println!("File contents: {}", content),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

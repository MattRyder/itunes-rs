extern crate byteorder;

pub mod hdfm;

use std::env;

fn create_library(file: String) -> hdfm::File {
    let mut library = hdfm::File::new(file);

    match library.process() {
        Ok(res) => {
            println!("Success! {}", res);
        },
        Err(message) => {
            println!("Failure: {}", message);
        }
    };

    library
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_arg = args.get(1);

    match path_arg {
        Some(path) => {
            let file_path = path.to_string();
            create_library(file_path);
        },
        _ => ()
    }
}
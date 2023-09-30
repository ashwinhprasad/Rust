use std::{fs::File, io::ErrorKind};

fn main() {

    // tries to open file, else throws the error
    let f = File::open("hello.txt").unwrap();


    // tries to open file, else throws the specified error
    let f : File = File::open("hello.txt").expect("Unable to find File");


   let f = match File::open("hello.txt") {
        Ok(file) => file,
        Err( ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(file) => file,
                Err(e) => {
                    panic!("Error while creating file");
                }
            }
        },
        Err(error)=> {
            panic!("Error while opening file");
        }
   };
}

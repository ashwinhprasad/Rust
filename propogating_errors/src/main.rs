use std::io;
use std::fs::File;


fn main() {
    match open_the_file() {
        Ok(file)=> {
            print!("File opened")
        }

        Err(err) => {
            panic!("Not opened")
        }
    }
}


fn open_the_file() -> Result<File,io::Error> {
    let f = File::open("heltlo.txt")?;
    return Ok(f);
}

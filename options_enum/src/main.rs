fn main() {
    
    let mut name: Option<String> = None;
    let age = 19;

    if age > 18 {
        name = Some(String::from("Ashwin"))
    }


    println!("{}",match name {

        Some(name) => {
            name
        }

        None => {
            String::from("Name not found")
        }

    })


}

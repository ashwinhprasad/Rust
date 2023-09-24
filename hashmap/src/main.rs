use std::collections::HashMap;

fn main() {
    let mut s1 = HashMap::new();
    s1.insert(String::from("name"), String::from("Ashwin Prasad"));
    s1.insert(String::from("age"), String::from("18"));
    s1.insert(String::from("college"), String::from("REC"));
    s1.insert(String::from("aka"), String::from("Batman"));
    let result = s1.get(&String::from("name"));
    let result = match result {
        Some(name) => {
            String::from(name)
        }
        None => {
            String::from("")
        }
    };
    println!("{}", result);
}

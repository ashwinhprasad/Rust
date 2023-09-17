fn main() {
    
    let mut names : Vec<String> = Vec::new();

    names.push(String::from("Ashwin"));
    names.push(String::from("Alex"));
    names.push(String::from("Andres"));
    names.pop();
    names.push(String::from("Harry"));
    names.remove(0);

    for i in 0..names.len() {
        println!("{}",names[i]);
    }
}

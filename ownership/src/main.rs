fn main() {
    
    let s1 : String = String::from("Hello");
    modify_string(s1); // move

    

    let s2 : String = String::from("Hello");
    modify_string1(s2.clone()); // pass by value
    println!("{}",s2);


    let mut s3 : String = String::from("Hello");
    modify_string2(&mut s3); // pass by reference
    println!("{}",s3);


}


fn modify_string(mut s1: String) {
    s1.push_str(" World");
    println!("{}",s1);
}



fn modify_string1(mut s : String) {
    s.push_str(" World");
    println!("{}",s);
}



fn modify_string2(s3:&mut String) {
    s3.push_str(" World");
}
struct Student {
    name: String,
    age : i32,
    email: String
}


impl Student {
    
    fn display_student_details(&self) {
        println!("{} of age {} has the email {}",self.name,self.age,self.email);
    }


    fn whos_older(s1 : &Student, s2 : &Student) -> bool {
        s1.age > s2.age
    }
}



fn main() {

    let s1 = Student{
        name: String::from("Patrick Jane"),
        age: 33,
        email: String::from("thementalist@gmail.com")
    };


    let s2 = Student{
        name: String::from("Rust Cohle"),
        age: 34,
        email: String::from("true.detective@gmail.com")
    };


    s1.display_student_details();
    s2.display_student_details();


    println!("Is Patrick Older ? {}",Student::whos_older(&s1, &s2));
}

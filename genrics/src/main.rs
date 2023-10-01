struct Person<U>{
    name:String,
    prop:U
}

fn main() {
   
    let var1 = return_the_given_variable(1);
    let var2 = return_the_given_variable("var2");

    let var3 = Person{
        name:String::from("John Doe"),
        prop:32
    };

    let var4 = Person{
        name:String::from("Jane Doe"),
        prop:"random str"
    };
}


fn return_the_given_variable<T>(var : T) -> T {
    var
}
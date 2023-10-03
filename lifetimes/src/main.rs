fn main() {
    let a : i32 = 34;
    let c;
    {
        let b : i32 = 45;
        c = lifetimes_example_func(&a, &b);
    }
}

/*
The function signature now tells Rust that for some lifetime 'a, 
the function takes two parameters, both of which are string slices 
that live at least as long as lifetime 'a. The function signature also tells 
Rust that the string slice returned from the function will live at least as 
long as lifetime of the param with the shorter lifetime passed as argument to the function.
This signature tells that the returned reference will live at least as long as the
param with the shorter lifetime
*/
fn lifetimes_example_func<'a>(first_number : &'a i32, second_number :&'a i32) -> &'a i32 {
    if first_number > second_number {
        first_number
    } else {
        second_number
    }
}
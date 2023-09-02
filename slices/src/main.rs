fn main() {

    let s1 = String::from("Hello");
    print_first_word(&s1);


    let arr1 = [1,2,3,4,5];
    let slice2 = &arr1[..];
    print!("{}",slice2[1]);

}


fn print_first_word(s1 : &str) {

    let mut end_idx: usize = 0;

    for (i,&character) in s1.as_bytes().iter().enumerate() {
        if character == b' ' {
            end_idx = i;
        }       
    }

    if end_idx == 0 {
        end_idx  = s1.len();
    }

    print!("{}",&s1[..end_idx]);

}

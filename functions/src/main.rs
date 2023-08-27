use std::io;

fn main(){

    loop {

        println!("1. Add");
        println!("2. Subtract");
        println!("3. Divide");
        println!("4. Multiply");
        println!("5. Exit");
        println!("Enter your choice");
        let mut ch = String::new();

        io::stdin().read_line(&mut ch)
                    .expect("Error occurred while reading input");


        let ch : i32 = match ch.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Kindly enter a number");
                continue;
            }
        };

        if ch == 5 {
            println!("Program Terminated");
            break;
        }



        let mut c : i32 = 0;
        let mut a = String::new();
        let mut b = String::new();


        println!("Enter the first number");
        io::stdin().read_line(&mut a).expect("Error occurred while reading input A");
        println!("Enter the second number");
        io::stdin().read_line(&mut b).expect("Error occurred while reading input B");

        let a : i32 = match a.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Kindly enter a number");
                continue;
            }
        };

        let b : i32 = match b.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Kindly enter a number");
                continue;
            }
        };



        match ch {
            1 => {
                c = add(a,b);
            },
            2 => {
                c = subtract(a,b);
            },
            3 => {
                c = divide(a,b);
            },
            4 => {
                c = multiply(a,b);
            },
            _ => {println!("Kindly enter a proper value");}            
        }


        println!("The Result : {}",c);

    }
}



fn add(a : i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a-b
}

fn divide(a: i32, b: i32) -> i32 {
    a/b
}

fn multiply(a: i32, b: i32) -> i32 {
    a*b
}
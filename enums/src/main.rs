enum ZohoProducts {
    Analytics,
    Dataprep,
    Writer
}


enum Message {
    SENT,
    SEEN(String)
}



fn main() {


    let p1 = ZohoProducts::Analytics;
    
    match p1 {
        
        ZohoProducts::Analytics => {
            print!("Analytics enum has been matched\n");
        }

        _ => {
            print!("This is not analytics\n");
        }

    }


    let message_status = Message::SEEN(String::from("Ashwin Prasad"));
    display_message(message_status);

}


fn display_message(msg: Message) {

    match msg {

        Message::SEEN(person) => {
            print!("read by {}",person);
        }

        _ => {

        }
    }
}
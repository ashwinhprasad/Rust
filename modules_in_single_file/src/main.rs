mod house {

    pub mod bedroom { 

        pub fn is_light_on() -> bool {
            true
        }

        pub fn is_study_room_light_on() -> bool {
            use super::study_room;
            study_room::is_light_on()
        }

    }


    pub mod study_room {

        pub fn is_light_on() -> bool {
            false
        }
    }


    pub fn get_house_number() -> i32 {
        43
    }

}


use crate::house::bedroom;

fn main() {
    println!("Is the light on in the bedroom of house-{} : {}",house::get_house_number(),bedroom::is_light_on());
    println!("Is the light on in the study room: {}",bedroom::is_study_room_light_on());
}

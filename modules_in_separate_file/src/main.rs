mod house;

use crate::house::bedroom;

fn main() {
    println!("Is the light on in the bedroom of house-{} : {}",house::get_house_number(),bedroom::is_light_on());
    println!("Is the light on in the study room: {}",bedroom::is_study_room_light_on());
}
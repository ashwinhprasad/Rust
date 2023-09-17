pub fn is_light_on() -> bool {
    true
}

pub fn is_study_room_light_on() -> bool {
    use super::studyroom;
    studyroom::is_light_on()
}

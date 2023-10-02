
trait Vehicle {
    fn get_max_speed(&self) -> i32;
    fn get_max_mileage(&self) -> i32;
    fn getPrice() -> i64 {
        return 3000;
    }
}


struct Car {
    name:String,
    speed:i32,
    mileage:i32,
    price:i64
}

struct Airplane {
    name:String,
    speed:i32,
    mileage:i32,
    price:i64,
    release_year:i32
}

impl Vehicle for Airplane {
    fn get_max_speed(&self) -> i32{
        self.speed + 200
    }
    fn get_max_mileage(&self) -> i32{
        self.mileage - 20
    }
}



impl Vehicle for Car {
    fn get_max_speed(&self) -> i32{
        self.speed + 2
    }
    fn get_max_mileage(&self) -> i32{
        self.mileage - 5
    }
}


fn main() {
    let ambassador : Car = Car {
        name:String::from("Ambassador"),
        speed:60,
        mileage:50,
        price:120000
    };

    let boeing : Airplane = Airplane {
        name:String::from("Ambassador"),
        speed:60,
        mileage:50,
        price:42120000,
        release_year: 2012
    };


    display_speed_and_mileage_of_vehicle(ambassador);
    display_speed_and_mileage_of_vehicle(boeing);
}


fn display_speed_and_mileage_of_vehicle<T: Vehicle>(vehicle: T) {
    print!("mileage is {} whereas the speed is {}",vehicle.get_max_mileage(),vehicle.get_max_speed())
}

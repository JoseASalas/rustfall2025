struct ParkingSystem {
    big: i32,
    medium: i32,
    small: i32
}


impl ParkingSystem {

    fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem{
            big: big,
            medium: medium,
            small: small
        }
    }
    
    fn add_car(&mut self, car_type: i32) -> bool {
        match car_type {
            1 => { self.big -= 1; self.big > -1},
            2 => { self.medium -= 1; self.medium > -1},
            3 => { self.small -= 1; self.small > -1},
            _ => false
        }
    }
}


fn main() {
    let mut service = ParkingSystem::new(1, 1, 0);
    println!("{}", service.add_car(1));
    println!("{}", service.add_car(2));
    println!("{}", service.add_car(3));
    println!("{}", service.add_car(1));
}

pub mod naive;

pub trait ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self;
    fn add_car(&mut self, car_type: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::ParkingSystem;

    pub fn run<P: ParkingSystem>() {
        let test_cases = [((1, 1, 0), &[(1, true), (2, true), (3, false), (1, false)] as &[_])];

        for ((big, medium, small), operations) in test_cases {
            let mut parking_system = P::new(big, medium, small);

            for &(car_type, expected) in operations {
                assert_eq!(parking_system.add_car(car_type), expected);
            }
        }
    }
}

pub mod standard;

pub trait UndergroundSystem {
    fn new() -> Self;
    fn check_in(&mut self, id: i32, station_name: String, t: i32);
    fn check_out(&mut self, id: i32, station_name: String, t: i32);
    fn get_average_time(&self, start_station: String, end_station: String) -> f64;
}

#[cfg(test)]
mod tests {
    use super::UndergroundSystem;

    enum Operation {
        CheckIn(i32, &'static str, i32),
        CheckOut(i32, &'static str, i32),
        GetAverageTime((&'static str, &'static str), f64),
    }

    pub fn run<S: UndergroundSystem>() {
        let test_cases = [
            &[
                Operation::CheckIn(45, "Leyton", 3),
                Operation::CheckIn(32, "Paradise", 8),
                Operation::CheckIn(27, "Leyton", 10),
                Operation::CheckOut(45, "Waterloo", 15),
                Operation::CheckOut(27, "Waterloo", 20),
                Operation::CheckOut(32, "Cambridge", 22),
                Operation::GetAverageTime(("Paradise", "Cambridge"), 14.0),
                Operation::GetAverageTime(("Leyton", "Waterloo"), 11.0),
                Operation::CheckIn(10, "Leyton", 24),
                Operation::GetAverageTime(("Leyton", "Waterloo"), 11.0),
                Operation::CheckOut(10, "Waterloo", 38),
                Operation::GetAverageTime(("Leyton", "Waterloo"), 12.0),
            ] as &[_],
            &[
                Operation::CheckIn(10, "Leyton", 3),
                Operation::CheckOut(10, "Paradise", 8),
                Operation::GetAverageTime(("Leyton", "Paradise"), 5.0),
                Operation::CheckIn(5, "Leyton", 10),
                Operation::CheckOut(5, "Paradise", 16),
                Operation::GetAverageTime(("Leyton", "Paradise"), 5.5),
                Operation::CheckIn(2, "Leyton", 21),
                Operation::CheckOut(2, "Paradise", 30),
                Operation::GetAverageTime(("Leyton", "Paradise"), 20.0 / 3.0),
            ],
        ];

        for operations in test_cases {
            let mut underground_system = S::new();

            for operation in operations {
                match *operation {
                    Operation::CheckIn(id, station_name, t) => {
                        underground_system.check_in(id, station_name.to_string(), t);
                    }
                    Operation::CheckOut(id, station_name, t) => {
                        underground_system.check_out(id, station_name.to_string(), t);
                    }
                    Operation::GetAverageTime((start_station, end_station), expected) => approx::assert_ulps_eq!(
                        underground_system.get_average_time(start_station.to_string(), end_station.to_string()),
                        expected,
                    ),
                }
            }
        }
    }
}

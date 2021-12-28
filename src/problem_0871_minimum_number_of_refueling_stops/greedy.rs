pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::BinaryHeap;
use std::convert::TryInto;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut passed_stations = BinaryHeap::<i32>::new();
        let mut prev_position = 0;
        let mut prev_fuel = start_fuel;

        for station in &stations {
            let [position, fuel]: [_; 2] = station.as_slice().try_into().unwrap();
            let fuel_required = position - prev_position;

            while prev_fuel < fuel_required {
                if let Some(fuel) = passed_stations.pop() {
                    prev_fuel += fuel;
                } else {
                    return -1;
                }
            }

            passed_stations.push(fuel);
            prev_position = position;
            prev_fuel -= fuel_required;
        }

        let fuel_required = target - prev_position;

        while prev_fuel < fuel_required {
            if let Some(fuel) = passed_stations.pop() {
                prev_fuel += fuel;
            } else {
                return -1;
            }
        }

        (stations.len() - passed_stations.len()) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        Self::min_refuel_stops(target, start_fuel, stations)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

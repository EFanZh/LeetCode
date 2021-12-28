pub mod greedy;

pub trait Solution {
    fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((1, 1, &[] as &[[_; 2]]), 0),
            ((100, 1, &[[10, 100]]), -1),
            ((100, 10, &[[10, 60], [20, 30], [30, 30], [60, 40]]), 2),
            ((100, 1, &[]), -1),
        ];

        for ((target, start_fuel, stations), expected) in test_cases {
            assert_eq!(
                S::min_refuel_stops(target, start_fuel, Matrix::to_vec(stations)),
                expected
            );
        }
    }
}

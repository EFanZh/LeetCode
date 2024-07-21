pub mod iterative;

pub trait Solution {
    fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4] as &[_], 0, 1), 1),
            ((&[1, 2, 3, 4], 0, 2), 3),
            ((&[1, 2, 3, 4], 0, 3), 4),
            ((&[7, 10, 1, 12, 11, 14, 5, 0], 7, 2), 17),
        ];

        for ((distance, start, destination), expected) in test_cases {
            assert_eq!(
                S::distance_between_bus_stops(distance.to_vec(), start, destination),
                expected,
            );
        }
    }
}

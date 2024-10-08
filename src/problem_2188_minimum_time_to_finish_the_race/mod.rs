pub mod dynamic_programming;

pub trait Solution {
    fn minimum_finish_time(tires: Vec<Vec<i32>>, change_time: i32, num_laps: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[2, 3], [3, 4]] as &[_], 5, 4), 21),
            ((&[[1, 10], [2, 2], [3, 4]], 6, 5), 25),
            (
                (&[[1, 2], [1, 3], [1, 4], [1, 5], [1, 6], [1, 7]], 100_000, 1000),
                8_214_040,
            ),
            ((&[[100_000, 100_000]], 100_000, 1000), 199_900_000),
        ];

        for ((tires, change_time, num_laps), expected) in test_cases {
            assert_eq!(
                S::minimum_finish_time(tires.iter().map(Vec::from).collect(), change_time, num_laps),
                expected,
            );
        }
    }
}

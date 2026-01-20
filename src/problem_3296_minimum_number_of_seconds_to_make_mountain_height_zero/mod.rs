pub mod binary_search;

pub trait Solution {
    fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((4, &[2, 1, 1] as &[_]), 3), ((10, &[3, 2, 2, 4]), 12), ((5, &[1]), 15)];

        for ((mountain_height, worker_times), expected) in test_cases {
            assert_eq!(
                S::min_number_of_seconds(mountain_height, worker_times.to_vec()),
                expected,
            );
        }
    }
}

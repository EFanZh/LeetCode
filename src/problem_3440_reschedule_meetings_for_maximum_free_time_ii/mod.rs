pub mod iterative;

pub trait Solution {
    fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((5, &[1, 3] as &[_], &[2, 5] as &[_]), 2),
            ((10, &[0, 7, 9], &[1, 8, 10]), 7),
            ((10, &[0, 3, 7, 9], &[1, 4, 8, 10]), 6),
            ((5, &[0, 1, 2, 3, 4], &[1, 2, 3, 4, 5]), 0),
        ];

        for ((event_time, start_time, end_time), expected) in test_cases {
            assert_eq!(
                S::max_free_time(event_time, start_time.to_vec(), end_time.to_vec()),
                expected,
            );
        }
    }
}

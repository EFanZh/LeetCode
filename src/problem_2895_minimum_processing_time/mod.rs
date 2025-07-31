pub mod greedy;

pub trait Solution {
    fn min_processing_time(processor_time: Vec<i32>, tasks: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[8, 10] as &[_], &[2, 2, 3, 1, 8, 7, 4, 5] as &[_]), 16),
            ((&[10, 20], &[2, 3, 1, 2, 5, 8, 4, 3]), 23),
        ];

        for ((processor_time, tasks), expected) in test_cases {
            assert_eq!(
                S::min_processing_time(processor_time.to_vec(), tasks.to_vec()),
                expected,
            );
        }
    }
}

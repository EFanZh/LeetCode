pub mod iterative;

pub trait Solution {
    fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[1, 2, 3, 3] as &[_], &[3, 4, 5, 6] as &[_], &[50, 10, 40, 70] as &[_]),
                120,
            ),
            ((&[1, 2, 3, 4, 6], &[3, 5, 10, 6, 9], &[20, 20, 100, 70, 60]), 150),
            ((&[1, 1, 1], &[2, 3, 4], &[5, 6, 4]), 6),
        ];

        for ((start_time, end_time, profit), expected) in test_cases {
            assert_eq!(
                S::job_scheduling(start_time.to_vec(), end_time.to_vec(), profit.to_vec()),
                expected,
            );
        }
    }
}

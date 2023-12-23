pub mod binary_search;
pub mod dynamic_programming;

pub trait Solution {
    fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 2, 3] as &[_], 3), 3),
            ((&[1, 2, 4, 7, 8], 2), 11),
            ((&[5, 5, 4, 4, 4], 2), 12),
        ];

        for ((jobs, k), expected) in test_cases {
            assert_eq!(S::minimum_time_required(jobs.to_vec(), k), expected);
        }
    }
}

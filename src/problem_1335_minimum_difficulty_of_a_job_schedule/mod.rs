pub mod dynamic_programming;

pub trait Solution {
    fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[6, 5, 4, 3, 2, 1] as &[_], 2), 7),
            ((&[9, 9, 9], 4), -1),
            ((&[1, 1, 1], 3), 3),
            ((&[11, 111, 22, 222, 33, 333, 44, 444], 6), 843),
        ];

        for ((job_difficulty, d), expected) in test_cases {
            assert_eq!(S::min_difficulty(job_difficulty.to_vec(), d), expected);
        }
    }
}

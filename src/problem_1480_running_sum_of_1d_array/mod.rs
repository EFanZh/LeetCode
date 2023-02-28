pub mod iterative;

pub trait Solution {
    fn running_sum(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 4] as &[_], &[1, 3, 6, 10] as &[_]),
            (&[1, 1, 1, 1, 1], &[1, 2, 3, 4, 5]),
            (&[3, 1, 2, 10, 1], &[3, 4, 6, 16, 17]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::running_sum(nums.to_vec()), expected);
        }
    }
}

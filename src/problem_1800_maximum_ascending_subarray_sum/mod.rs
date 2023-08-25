pub mod iterative;

pub trait Solution {
    fn max_ascending_sum(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[10, 20, 30, 5, 10, 50] as &[_], 65),
            (&[10, 20, 30, 40, 50], 150),
            (&[12, 17, 15, 13, 10, 11, 12], 33),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_ascending_sum(nums.to_vec()), expected);
        }
    }
}

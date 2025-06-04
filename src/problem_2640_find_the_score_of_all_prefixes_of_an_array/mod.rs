pub mod iterative;

pub trait Solution {
    fn find_prefix_score(nums: Vec<i32>) -> Vec<i64>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 3, 7, 5, 10] as &[_], &[4_i64, 10, 24, 36, 56] as &[_]),
            (&[1, 1, 2, 4, 8, 16], &[2, 4, 8, 16, 32, 64]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::find_prefix_score(nums.to_vec()), expected);
        }
    }
}

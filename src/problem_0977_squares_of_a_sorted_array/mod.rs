pub mod merge;

pub trait Solution {
    fn sorted_squares(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[-4, -1, 0, 3, 10] as &[_], &[0, 1, 9, 16, 100] as &[_]),
            (&[-7, -3, 2, 3, 11], &[4, 9, 9, 49, 121]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::sorted_squares(nums.to_vec()), expected);
        }
    }
}

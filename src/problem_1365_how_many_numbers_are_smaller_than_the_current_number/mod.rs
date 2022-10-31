pub mod hash_map;

pub trait Solution {
    fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[8, 1, 2, 2, 3] as &[_], &[4, 0, 1, 1, 3] as &[_]),
            (&[6, 5, 4, 8], &[2, 1, 0, 3]),
            (&[7, 7, 7, 7], &[0, 0, 0, 0]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::smaller_numbers_than_current(nums.to_vec()), expected);
        }
    }
}

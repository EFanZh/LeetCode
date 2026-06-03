pub mod iterative;

pub trait Solution {
    fn find_missing_elements(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 4, 2, 5] as &[_], &[3] as &[_]),
            (&[7, 8, 6, 9], &[]),
            (&[5, 1], &[2, 3, 4]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::find_missing_elements(nums.to_vec()), expected);
        }
    }
}

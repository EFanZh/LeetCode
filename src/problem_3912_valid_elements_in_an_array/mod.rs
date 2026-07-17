pub mod iterative;

pub trait Solution {
    fn find_valid_elements(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 4, 2, 3, 2] as &[_], &[1, 2, 4, 3, 2] as &[_]),
            (&[5, 5, 5, 5], &[5, 5]),
            (&[1], &[1]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::find_valid_elements(nums.to_vec()), expected);
        }
    }
}

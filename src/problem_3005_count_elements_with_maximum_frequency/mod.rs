pub mod iterative;

pub trait Solution {
    fn max_frequency_elements(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 2, 3, 1, 4] as &[_], 4), (&[1, 2, 3, 4, 5], 5)];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_frequency_elements(nums.to_vec()), expected);
        }
    }
}

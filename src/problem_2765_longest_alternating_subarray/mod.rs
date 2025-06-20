pub mod sliding_window;

pub trait Solution {
    fn alternating_subarray(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 3, 4, 3, 4] as &[_], 4), (&[4, 5, 6], 2)];

        for (nums, expected) in test_cases {
            assert_eq!(S::alternating_subarray(nums.to_vec()), expected);
        }
    }
}

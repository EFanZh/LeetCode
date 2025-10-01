pub mod sliding_window;
pub mod sliding_window_2;

pub trait Solution {
    fn continuous_subarrays(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[5, 4, 2, 4] as &[_], 8), (&[1, 2, 3], 6)];

        for (nums, expected) in test_cases {
            assert_eq!(S::continuous_subarrays(nums.to_vec()), expected);
        }
    }
}

pub mod sliding_window;

pub trait Solution {
    fn count_complete_subarrays(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 3, 1, 2, 2] as &[_], 4), (&[5, 5, 5, 5], 10)];

        for (nums, expected) in test_cases {
            assert_eq!(S::count_complete_subarrays(nums.to_vec()), expected);
        }
    }
}

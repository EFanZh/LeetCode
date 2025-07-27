pub mod greedy;

pub trait Solution {
    fn max_subarrays(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 0, 2, 0, 1, 2] as &[_], 3), (&[5, 7, 1, 3], 1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_subarrays(nums.to_vec()), expected);
        }
    }
}

pub mod mathematical;

pub trait Solution {
    fn count_alternating_subarrays(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[0, 1, 1, 1] as &[_], 5), (&[1, 0, 1, 0], 10)];

        for (nums, expected) in test_cases {
            assert_eq!(S::count_alternating_subarrays(nums.to_vec()), expected);
        }
    }
}

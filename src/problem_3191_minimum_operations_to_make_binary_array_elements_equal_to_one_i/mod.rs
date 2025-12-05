pub mod greedy;

pub trait Solution {
    fn min_operations(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[0, 1, 1, 1, 0, 0] as &[_], 3), (&[0, 1, 1, 1], -1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::min_operations(nums.to_vec()), expected);
        }
    }
}

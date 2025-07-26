pub mod hash_map;

pub trait Solution {
    fn min_operations(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 3, 3, 2, 2, 4, 2, 3, 4] as &[_], 4), (&[2, 1, 2, 2, 3, 3], -1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::min_operations(nums.to_vec()), expected);
        }
    }
}

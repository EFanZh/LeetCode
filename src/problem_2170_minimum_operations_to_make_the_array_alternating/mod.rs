pub mod hash_map;

pub trait Solution {
    fn minimum_operations(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 1, 3, 2, 4, 3] as &[_], 3),
            (&[1, 2, 2, 2, 2], 2),
            (&[3, 1, 3, 2, 4, 3], 3),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::minimum_operations(nums.to_vec()), expected);
        }
    }
}

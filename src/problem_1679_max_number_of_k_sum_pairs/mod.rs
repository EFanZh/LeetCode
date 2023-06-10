pub mod hash_map;

pub trait Solution {
    fn max_operations(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4] as &[_], 5), 2),
            ((&[3, 1, 3, 4, 3], 6), 1),
            ((&[2, 5, 4, 4, 1, 3, 4, 4, 1, 4, 4, 1, 2, 1, 2, 2, 3, 2, 4, 2], 3), 4),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::max_operations(nums.to_vec(), k), expected);
        }
    }
}

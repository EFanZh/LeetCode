pub mod dynamic_programming;

pub trait Solution {
    fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 1, 4] as &[_], 2), 4),
            ((&[6, 3, 8, 1, 3, 1, 2, 2], 4), 6),
            ((&[5, 3, 3, 6, 3, 3], 3), -1),
            ((&[7, 3, 16, 15, 1, 13, 1, 2, 14, 5, 3, 10, 6, 2, 7, 15], 8), 12),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::minimum_incompatibility(nums.to_vec(), k), expected);
        }
    }
}

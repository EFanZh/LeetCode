pub mod bit_masks;

pub trait Solution {
    fn min_operations(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[5, 2, 5, 4, 5] as &[_], 2), 2),
            ((&[2, 1, 2], 2), -1),
            ((&[9, 7, 5, 3], 1), 4),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::min_operations(nums.to_vec(), k), expected);
        }
    }
}

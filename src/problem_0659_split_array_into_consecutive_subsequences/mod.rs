pub mod check_by_chunks;
pub mod two_binary_heaps;
pub mod two_optimized_binary_heaps;

pub trait Solution {
    fn is_possible(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 3, 4, 5] as &[_], true),
            (&[1, 2, 3, 3, 4, 4, 5, 5], true),
            (&[1, 2, 3, 4, 4, 5], false),
            (&[1, 2, 3, 5, 5, 6, 7], false),
            (&[1, 2, 5, 5, 6, 6, 7, 8, 8, 9], false),
            (&[4, 5, 6, 7, 7, 8, 8, 9, 10, 11], true),
            (&[2, 5, 5, 5, 6, 7, 8, 8, 8, 9], false),
            (&[1], false),
            (&[5, 5, 6, 10, 10, 11, 12, 13, 14, 15], false),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::is_possible(nums.to_vec()), expected);
        }
    }
}

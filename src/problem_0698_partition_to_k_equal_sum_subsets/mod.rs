pub mod backtracking;

pub trait Solution {
    fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[4, 3, 2, 3, 5, 2, 1] as &[_], 4), true),
            ((&[1, 2, 3, 4], 3), false),
            ((&[10, 12, 1, 2, 10, 7, 5, 19, 13, 1], 4), true),
            ((&[2, 3, 5, 7], 1), true),
            ((&[1, 9, 8], 3), false),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::can_partition_k_subsets(nums.to_vec(), k), expected);
        }
    }
}

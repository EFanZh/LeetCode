pub mod iterative;

pub trait Solution {
    fn min_operations(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 1, 5, 4, 2] as &[_], 2), 4),
            ((&[3, 1, 5, 4, 2], 5), 5),
            ((&[3, 2, 5, 3, 1], 3), 4),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::min_operations(nums.to_vec(), k), expected);
        }
    }
}

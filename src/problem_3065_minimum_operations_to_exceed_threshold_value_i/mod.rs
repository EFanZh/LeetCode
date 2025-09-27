pub mod iterative;

pub trait Solution {
    fn min_operations(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 11, 10, 1, 3] as &[_], 10), 3),
            ((&[1, 1, 2, 4, 9], 1), 0),
            ((&[1, 1, 2, 4, 9], 9), 4),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::min_operations(nums.to_vec(), k), expected);
        }
    }
}

pub mod dynamic_programming;

pub trait Solution {
    fn min_increment_operations(nums: Vec<i32>, k: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 3, 0, 0, 2] as &[_], 4), 3),
            ((&[0, 1, 3, 3], 5), 2),
            ((&[1, 1, 2], 1), 0),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::min_increment_operations(nums.to_vec(), k), expected);
        }
    }
}

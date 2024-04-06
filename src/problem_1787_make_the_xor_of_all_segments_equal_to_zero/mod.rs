pub mod dynamic_programming;

pub trait Solution {
    fn min_changes(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 0, 3, 0] as &[_], 1), 3),
            ((&[3, 4, 5, 2, 1, 7, 3, 4, 7], 3), 3),
            ((&[1, 2, 4, 1, 2, 5, 1, 2, 6], 3), 3),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::min_changes(nums.to_vec(), k), expected);
        }
    }
}

pub mod mathematical;

pub trait Solution {
    fn min_operations(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[2, 1, 3, 4] as &[_], 1), 2), ((&[2, 0, 2, 0], 0), 0)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::min_operations(nums.to_vec(), k), expected);
        }
    }
}

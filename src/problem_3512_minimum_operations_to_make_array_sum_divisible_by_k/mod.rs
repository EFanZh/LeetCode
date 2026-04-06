pub mod brute_force;

pub trait Solution {
    fn min_operations(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[3, 9, 7] as &[_], 5), 4), ((&[4, 1, 3], 4), 0), ((&[3, 2], 6), 5)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::min_operations(nums.to_vec(), k), expected);
        }
    }
}

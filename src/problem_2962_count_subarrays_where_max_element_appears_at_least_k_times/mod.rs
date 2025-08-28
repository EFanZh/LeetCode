pub mod sliding_window;

pub trait Solution {
    fn count_subarrays(nums: Vec<i32>, k: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 3, 2, 3, 3] as &[_], 2), 6), ((&[1, 4, 2, 1], 3), 0)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::count_subarrays(nums.to_vec(), k), expected);
        }
    }
}

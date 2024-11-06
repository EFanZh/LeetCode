pub mod sliding_window;

pub trait Solution {
    fn count_subarrays(nums: Vec<i32>, k: i64) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[2, 1, 4, 3, 5] as &[_], 10), 6), ((&[1, 1, 1], 5), 5)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::count_subarrays(nums.to_vec(), k), expected);
        }
    }
}

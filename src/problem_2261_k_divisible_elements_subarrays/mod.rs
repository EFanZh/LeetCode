pub mod sliding_window_and_hash_set;

pub trait Solution {
    fn count_distinct(nums: Vec<i32>, k: i32, p: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[2, 3, 3, 2, 2] as &[_], 2, 2), 11), ((&[1, 2, 3, 4], 4, 1), 10)];

        for ((nums, k, p), expected) in test_cases {
            assert_eq!(S::count_distinct(nums.to_vec(), k, p), expected);
        }
    }
}

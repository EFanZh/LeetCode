pub mod sliding_window;

pub trait Solution {
    fn count_good(nums: Vec<i32>, k: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 1, 1, 1, 1] as &[_], 10), 1), ((&[3, 1, 4, 3, 2, 2, 4], 2), 4)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::count_good(nums.to_vec(), k), expected);
        }
    }
}

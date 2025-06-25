pub mod sliding_window;

pub trait Solution {
    fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[4, 6, 1, 2], 2), 3), ((&[1, 1, 1, 1], 10), 4)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::maximum_beauty(nums.to_vec(), k), expected);
        }
    }
}

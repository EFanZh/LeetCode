pub mod sliding_window;

pub trait Solution {
    fn minimum_difference(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[90] as &[_], 1), 0), ((&[9, 4, 1, 7], 2), 2)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::minimum_difference(nums.to_vec(), k), expected);
        }
    }
}

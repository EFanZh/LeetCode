pub mod dynamic_programming_fast;
pub mod dynamic_programming_slow;

pub trait Solution {
    fn length_of_lis(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run_tests<S: Solution>() {
        let test_cases = vec![(vec![10, 9, 2, 5, 3, 7, 101, 18], 4)];

        for (nums, expected) in test_cases {
            assert_eq!(S::length_of_lis(nums), expected);
        }
    }
}

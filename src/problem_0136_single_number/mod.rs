pub mod reduce_xor;

pub trait Solution {
    fn single_number(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run_tests<S: Solution>() {
        let test_cases = vec![(vec![2, 2, 1], 1), (vec![4, 1, 2, 1, 2], 4)];

        for (nums, expected) in test_cases {
            assert_eq!(S::single_number(nums), expected);
        }
    }
}

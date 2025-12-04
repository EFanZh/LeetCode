pub mod iterative;

pub trait Solution {
    fn minimum_operations(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3, 4] as &[_], 3), (&[3, 6, 9], 0)];

        for (nums, expected) in test_cases {
            assert_eq!(S::minimum_operations(nums.to_vec()), expected);
        }
    }
}

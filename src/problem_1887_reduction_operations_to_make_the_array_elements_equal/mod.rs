pub mod iterative;

pub trait Solution {
    fn reduction_operations(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[5, 1, 3] as &[_], 3), (&[1, 1, 1], 0), (&[1, 1, 2, 2, 3], 4)];

        for (nums, expected) in test_cases {
            assert_eq!(S::reduction_operations(nums.to_vec()), expected);
        }
    }
}

pub mod iterative;

pub trait Solution {
    fn min_deletions(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 1, 2, 2, 2, 3] as &[_], 3), (&[4, 4, 9, 4], 1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::min_deletions(nums.to_vec()), expected);
        }
    }
}

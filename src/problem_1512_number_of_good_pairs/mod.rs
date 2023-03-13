pub mod iterative;

pub trait Solution {
    fn num_identical_pairs(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3, 1, 1, 3] as &[_], 4), (&[1, 1, 1, 1], 6), (&[1, 2, 3], 0)];

        for (nums, expected) in test_cases {
            assert_eq!(S::num_identical_pairs(nums.to_vec()), expected);
        }
    }
}

pub mod iterative;

pub trait Solution {
    fn min_absolute_difference(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 0, 0, 2, 0, 1] as &[_], 2), (&[1, 0, 1, 0], -1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::min_absolute_difference(nums.to_vec()), expected);
        }
    }
}

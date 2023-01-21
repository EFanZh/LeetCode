pub mod iterative;

pub trait Solution {
    fn min_start_value(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[-3, 2, -3, 4, 2] as &[_], 5), (&[1, 2], 1), (&[1, -2, -3], 5)];

        for (nums, expected) in test_cases {
            assert_eq!(S::min_start_value(nums.to_vec()), expected);
        }
    }
}

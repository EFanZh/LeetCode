pub mod iterative;

pub trait Solution {
    fn maximum_difference(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[7, 1, 5, 4] as &[_], 4), (&[9, 4, 3, 2], -1), (&[1, 5, 2, 10], 9)];

        for (nums, expected) in test_cases {
            assert_eq!(S::maximum_difference(nums.to_vec()), expected);
        }
    }
}

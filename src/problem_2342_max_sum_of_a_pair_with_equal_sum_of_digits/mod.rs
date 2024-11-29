pub mod mathematical;

pub trait Solution {
    fn maximum_sum(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[18, 43, 36, 13, 7] as &[_], 54), (&[10, 12, 19, 14], -1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::maximum_sum(nums.to_vec()), expected);
        }
    }
}

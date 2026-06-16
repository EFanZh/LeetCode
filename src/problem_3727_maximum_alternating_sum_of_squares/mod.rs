pub mod quick_select;

pub trait Solution {
    fn max_alternating_sum(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3] as &[_], 12), (&[1, -1, 2, -2, 3, -3], 16)];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_alternating_sum(nums.to_vec()), expected);
        }
    }
}

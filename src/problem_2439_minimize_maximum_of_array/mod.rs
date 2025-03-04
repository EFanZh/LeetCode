pub mod iterative;

pub trait Solution {
    fn minimize_array_value(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[3, 7, 1, 6] as &[_], 5), (&[10, 1], 10)];

        for (nums, expected) in test_cases {
            assert_eq!(S::minimize_array_value(nums.to_vec()), expected);
        }
    }
}

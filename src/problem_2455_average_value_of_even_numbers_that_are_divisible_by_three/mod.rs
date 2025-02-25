pub mod iterative;

pub trait Solution {
    fn average_value(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 3, 6, 10, 12, 15] as &[_], 9), (&[1, 2, 4, 7, 10], 0)];

        for (nums, expected) in test_cases {
            assert_eq!(S::average_value(nums.to_vec()), expected);
        }
    }
}

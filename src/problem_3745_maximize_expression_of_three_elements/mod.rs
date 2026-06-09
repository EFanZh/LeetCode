pub mod iterative;

pub trait Solution {
    fn maximize_expression_of_three(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 4, 2, 5] as &[_], 8), (&[-2, 0, 5, -2, 4], 11)];

        for (nums, expected) in test_cases {
            assert_eq!(S::maximize_expression_of_three(nums.to_vec()), expected);
        }
    }
}

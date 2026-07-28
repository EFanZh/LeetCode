pub mod iterative;

pub trait Solution {
    fn max_digit_range(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[5724, 111, 350] as &[_], 6074), (&[90, 900], 990)];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_digit_range(nums.to_vec()), expected);
        }
    }
}

pub mod iterative;

pub trait Solution {
    fn missing_integer(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3, 2, 5] as &[_], 6), (&[3, 4, 5, 1, 12, 14, 13], 15)];

        for (nums, expected) in test_cases {
            assert_eq!(S::missing_integer(nums.to_vec()), expected);
        }
    }
}

pub mod iterative;

pub trait Solution {
    fn is_monotonic(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 2, 3] as &[_], true),
            (&[6, 5, 4, 4], true),
            (&[1, 3, 2], false),
            (&[5, 3, 2, 4, 1], false),
            (&[1, 1, 1], true),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::is_monotonic(nums.to_vec()), expected);
        }
    }
}

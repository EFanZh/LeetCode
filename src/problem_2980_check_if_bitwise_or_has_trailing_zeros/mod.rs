pub mod greedy;

pub trait Solution {
    fn has_trailing_zeros(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 4, 5] as &[_], true),
            (&[2, 4, 8, 16], true),
            (&[1, 3, 5, 7, 9], false),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::has_trailing_zeros(nums.to_vec()), expected);
        }
    }
}

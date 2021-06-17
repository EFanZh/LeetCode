pub mod pigeonhole;

pub trait Solution {
    fn first_missing_positive(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 0] as &[_], 3),
            (&[3, 4, -1, 1], 2),
            (&[7, 8, 9, 11, 12], 1),
            (&[], 1),
            (&[1], 2),
            (&[1, 1], 2),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::first_missing_positive(nums.to_vec()), expected);
        }
    }
}

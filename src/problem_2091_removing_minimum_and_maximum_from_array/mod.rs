pub mod iterative;

pub trait Solution {
    fn minimum_deletions(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 10, 7, 5, 4, 1, 8, 6] as &[_], 5),
            (&[0, -4, 19, 1, 8, -2, -3, 5], 3),
            (&[101], 1),
            (&[42, -75], 2),
            (&[-14, 61, 29, -18, 59, 13, -67, -16, 55, -57, 7, 74], 6),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::minimum_deletions(nums.to_vec()), expected);
        }
    }
}

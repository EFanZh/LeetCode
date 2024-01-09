pub mod iterative;

pub trait Solution {
    fn smallest_equal(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[0, 1, 2] as &[_], 0),
            (&[4, 3, 2, 1], 2),
            (&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0], -1),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::smallest_equal(nums.to_vec()), expected);
        }
    }
}

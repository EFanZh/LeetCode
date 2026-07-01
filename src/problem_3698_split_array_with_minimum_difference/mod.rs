pub mod iterative;

pub trait Solution {
    fn split_array(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 3, 2] as &[_], 2),
            (&[1, 2, 4, 3], 4),
            (&[3, 1, 2], -1),
            (&[29, 30, 28, 27, 21, 18], 35),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::split_array(nums.to_vec()), expected);
        }
    }
}

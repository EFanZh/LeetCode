pub mod iterative;

pub trait Solution {
    fn maximum_count(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[-2, -1, -1, 1, 2, 3] as &[_], 3),
            (&[-3, -2, -1, 0, 0, 1, 2], 3),
            (&[5, 20, 66, 1314], 4),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::maximum_count(nums.to_vec()), expected);
        }
    }
}

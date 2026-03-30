pub mod iterative;

pub trait Solution {
    fn max_sum(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 4, 5] as &[_], 15),
            (&[1, 1, 0, 1, 1], 1),
            (&[1, 2, -1, -2, 1, 0, -1], 3),
            (&[-100], -100),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_sum(nums.to_vec()), expected);
        }
    }
}

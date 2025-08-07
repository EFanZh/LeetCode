pub mod iterative;

pub trait Solution {
    fn minimum_sum(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[8, 6, 1, 5, 3] as &[_], 9),
            (&[5, 4, 8, 7, 10, 2], 13),
            (&[6, 5, 4, 3, 4, 5], -1),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::minimum_sum(nums.to_vec()), expected);
        }
    }
}

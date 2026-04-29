pub mod greedy;

pub trait Solution {
    fn min_swaps(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 4, 6, 5, 7] as &[_], 3),
            (&[2, 4, 5, 7], 1),
            (&[1, 2, 3], 0),
            (&[4, 5, 6, 8], -1),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::min_swaps(nums.to_vec()), expected);
        }
    }
}

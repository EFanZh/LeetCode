pub mod greedy;

pub trait Solution {
    fn minimum_replacement(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 9, 3] as &[_], 2),
            (&[1, 2, 3, 4, 5], 0),
            (&[12, 9, 7, 6, 17, 19, 21], 6),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::minimum_replacement(nums.to_vec()), expected);
        }
    }
}

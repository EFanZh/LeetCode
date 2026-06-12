pub mod brute_force;

pub trait Solution {
    fn sort_by_reflection(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[4, 5, 4] as &[_], &[4, 4, 5] as &[_]),
            (&[3, 6, 5, 8], &[8, 3, 6, 5]),
            (&[8, 2], &[2, 8]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::sort_by_reflection(nums.to_vec()), expected);
        }
    }
}

pub mod iterative;

pub trait Solution {
    fn min_moves(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3] as &[_], 3),
            (&[], 0),
            (&[83, 86, 77, 15, 93, 35, 86, 92, 49, 21], 487),
        ];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::min_moves(nums.to_vec()), expected);
        }
    }
}

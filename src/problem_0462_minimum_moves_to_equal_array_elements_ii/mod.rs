pub mod median;

pub trait Solution {
    fn min_moves2(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3] as &[_], 2),
            (&[1, 1, 1], 0),
            (&[], 0),
            (&[3], 0),
            (&[3, 2, 1], 2),
            (&[1, 3, 2], 2),
        ];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::min_moves2(nums.to_vec()), expected);
        }
    }
}

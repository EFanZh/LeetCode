pub mod iterative;

pub trait Solution {
    fn third_max(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 2, 1] as &[_], 1),
            (&[1, 2], 2),
            (&[2, 2, 3, 1], 1),
            (&[1, 2, 2], 2),
            (&[2, 4, 6, 1], 2),
            (&[2, 4, 6, 2], 2),
            (&[2, 4, 6, 3], 3),
            (&[2, 4, 6, 4], 2),
            (&[2, 4, 6, 5], 4),
            (&[2, 4, 6, 6], 2),
            (&[2, 4, 6, 7], 4),
        ];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::third_max(nums.to_vec()), expected);
        }
    }
}

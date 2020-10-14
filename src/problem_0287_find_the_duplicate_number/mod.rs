pub mod cycle_detection;

pub trait Solution {
    fn find_duplicate(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 3, 4, 2, 2] as &[_], 2),
            (&[3, 1, 3, 4, 2], 3),
            (&[1, 1], 1),
            (&[1, 1, 2], 1),
            (&[2, 2, 2, 2, 2], 2),
        ];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::find_duplicate(nums.to_vec()), expected);
        }
    }
}

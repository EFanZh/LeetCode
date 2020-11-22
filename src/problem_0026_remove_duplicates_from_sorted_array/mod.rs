pub mod cheating;
pub mod sequential_processing;

pub trait Solution {
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 1, 2] as &[_], &[1, 2] as &[_]),
            (&[0, 0, 1, 1, 1, 2, 2, 3, 3, 4], &[0, 1, 2, 3, 4]),
            (&[1], &[1]),
            (&[], &[]),
        ];

        for (nums, expected) in test_cases.iter().copied() {
            let mut nums = nums.to_vec();
            let result = S::remove_duplicates(&mut nums);

            assert_eq!(nums, expected);
            assert_eq!(result as usize, expected.len());
        }
    }
}

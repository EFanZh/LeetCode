pub mod cheating;
pub mod sequential_processing;
pub mod sequential_processing_2;

pub trait Solution {
    fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 2, 2, 3] as &[_], 3), &[2, 2] as &[_]),
            ((&[0, 1, 2, 2, 3, 0, 4, 2], 2), &[0, 1, 3, 0, 4]),
        ];

        for ((nums, val), expected) in test_cases.iter().copied() {
            let mut nums = nums.to_vec();
            let result = S::remove_element(&mut nums, val);

            assert_eq!(nums, expected);
            assert_eq!(result as usize, expected.len());
        }
    }
}

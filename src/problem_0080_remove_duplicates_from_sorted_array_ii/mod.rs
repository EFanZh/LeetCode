pub mod iterative;
pub mod recursive;

pub trait Solution {
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 1, 1, 2, 2, 3] as &[_], &[1, 1, 2, 2, 3] as &[_]),
            (&[0, 0, 1, 1, 1, 1, 2, 3, 3], &[0, 0, 1, 1, 2, 3, 3]),
        ];

        for (nums, expected) in test_cases.iter().copied() {
            let mut nums = nums.to_vec();

            assert_eq!(S::remove_duplicates(&mut nums), expected.len() as _);
            assert_eq!(nums, expected);
        }
    }
}

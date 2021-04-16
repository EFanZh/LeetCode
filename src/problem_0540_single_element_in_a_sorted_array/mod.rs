pub mod binary_search;
pub mod binary_search_2;

pub trait Solution {
    fn single_non_duplicate(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 1, 2, 3, 3, 4, 4, 8, 8] as &[_], 2),
            (&[3, 3, 7, 7, 10, 11, 11], 10),
            (&[1, 1, 2, 2, 3], 3),
        ];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::single_non_duplicate(nums.to_vec()), expected);
        }
    }
}

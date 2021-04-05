pub mod quick_select;

pub trait Solution {
    fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 1, 1, 2, 2, 3] as &[_], 2), &[1, 2] as &[_]),
            ((&[1], 1), &[1]),
            ((&[1, 2], 2), &[1, 2]),
            ((&[1, 1, 1, 2, 2, 3], 2), &[1, 2]),
            ((&[5, 2, 5, 3, 5, 3, 1, 1, 3], 2), &[3, 5]),
        ];

        for ((nums, k), expected) in test_cases.iter().copied() {
            assert_eq!(
                test_utilities::unstable_sorted(S::top_k_frequent(nums.to_vec(), k)),
                expected
            );
        }
    }
}

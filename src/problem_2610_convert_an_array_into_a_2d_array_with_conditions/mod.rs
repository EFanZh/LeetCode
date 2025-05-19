pub mod greedy;

pub trait Solution {
    fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;
    use std::collections::HashSet;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 3, 4, 1, 2, 3, 1] as &[_], 3), (&[1, 2, 3, 4], 1)];
        let mut buffer = HashSet::new();

        for (nums, expected) in test_cases {
            let result = S::find_matrix(nums.to_vec());

            assert_eq!(
                test_utilities::unstable_sorted(result.iter().flatten().copied()),
                test_utilities::unstable_sorted(nums.iter().copied()),
            );

            for row in &result {
                buffer.extend(row.iter().copied());

                assert_eq!(row.len(), buffer.len());

                buffer.clear();
            }

            assert_eq!(result.len(), expected);
        }
    }
}

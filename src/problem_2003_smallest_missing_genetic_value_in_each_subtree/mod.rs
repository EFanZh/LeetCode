pub mod bottom_up;

pub trait Solution {
    fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[-1, 0, 0, 2] as &[_], &[1, 2, 3, 4] as &[_]), &[5, 1, 1, 1] as &[_]),
            ((&[-1, 0, 1, 0, 3, 3], &[5, 4, 6, 2, 1, 3]), &[7, 1, 1, 4, 2, 1]),
            (
                (&[-1, 2, 3, 0, 2, 4, 1], &[2, 3, 4, 5, 6, 7, 8]),
                &[1, 1, 1, 1, 1, 1, 1],
            ),
        ];

        for ((parents, nums), expected) in test_cases {
            assert_eq!(
                S::smallest_missing_value_subtree(parents.to_vec(), nums.to_vec()),
                expected,
            );
        }
    }
}

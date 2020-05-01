pub mod backtracking_1;
pub mod backtracking_2;
pub mod backtracking_3;
pub mod next_permutations;

pub trait Solution {
    fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[] as &[_], &[&[] as &[_]] as &[&[_]]),
            (&[1], &[&[1]]),
            (&[1, 1], &[&[1, 1]]),
            (&[1, 2], &[&[1, 2], &[2, 1]]),
            (&[1, 1, 2], &[&[1, 1, 2], &[1, 2, 1], &[2, 1, 1]]),
            (
                &[2, 2, 1, 1],
                &[
                    &[1, 1, 2, 2],
                    &[1, 2, 1, 2],
                    &[1, 2, 2, 1],
                    &[2, 1, 1, 2],
                    &[2, 1, 2, 1],
                    &[2, 2, 1, 1],
                ],
            ),
            (
                &[3, 3, 0, 3],
                &[&[0, 3, 3, 3], &[3, 0, 3, 3], &[3, 3, 0, 3], &[3, 3, 3, 0]],
            ),
            (
                &[0, 1, 0, 0, 9],
                &[
                    &[0, 0, 0, 1, 9],
                    &[0, 0, 0, 9, 1],
                    &[0, 0, 1, 0, 9],
                    &[0, 0, 1, 9, 0],
                    &[0, 0, 9, 0, 1],
                    &[0, 0, 9, 1, 0],
                    &[0, 1, 0, 0, 9],
                    &[0, 1, 0, 9, 0],
                    &[0, 1, 9, 0, 0],
                    &[0, 9, 0, 0, 1],
                    &[0, 9, 0, 1, 0],
                    &[0, 9, 1, 0, 0],
                    &[1, 0, 0, 0, 9],
                    &[1, 0, 0, 9, 0],
                    &[1, 0, 9, 0, 0],
                    &[1, 9, 0, 0, 0],
                    &[9, 0, 0, 0, 1],
                    &[9, 0, 0, 1, 0],
                    &[9, 0, 1, 0, 0],
                    &[9, 1, 0, 0, 0],
                ],
            ),
        ];

        for (nums, expected) in test_cases.iter().copied() {
            let mut result = S::permute_unique(nums.to_vec());

            result.sort_unstable();

            assert_eq!(result, expected);
        }
    }
}

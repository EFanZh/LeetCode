pub mod backtracking_1;
pub mod backtracking_2;
pub mod heap;

pub trait Solution {
    fn permute(nums: Vec<i32>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[] as &[_], &[&[] as &[_]] as &[&[_]]),
            (&[1], &[&[1]]),
            (&[1, 2], &[&[1, 2], &[2, 1]]),
            (
                &[1, 2, 3],
                &[&[1, 2, 3], &[1, 3, 2], &[2, 1, 3], &[2, 3, 1], &[3, 1, 2], &[3, 2, 1]],
            ),
            (
                &[1, 2, 3, 4],
                &[
                    &[1, 2, 3, 4],
                    &[1, 2, 4, 3],
                    &[1, 3, 2, 4],
                    &[1, 3, 4, 2],
                    &[1, 4, 2, 3],
                    &[1, 4, 3, 2],
                    &[2, 1, 3, 4],
                    &[2, 1, 4, 3],
                    &[2, 3, 1, 4],
                    &[2, 3, 4, 1],
                    &[2, 4, 1, 3],
                    &[2, 4, 3, 1],
                    &[3, 1, 2, 4],
                    &[3, 1, 4, 2],
                    &[3, 2, 1, 4],
                    &[3, 2, 4, 1],
                    &[3, 4, 1, 2],
                    &[3, 4, 2, 1],
                    &[4, 1, 2, 3],
                    &[4, 1, 3, 2],
                    &[4, 2, 1, 3],
                    &[4, 2, 3, 1],
                    &[4, 3, 1, 2],
                    &[4, 3, 2, 1],
                ],
            ),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(test_utilities::unstable_sorted(S::permute(nums.to_vec())), expected);
        }
    }
}

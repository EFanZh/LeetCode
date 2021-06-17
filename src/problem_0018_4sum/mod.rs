pub mod reduce_to_three_sum;

pub trait Solution {
    fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[1, 0, -1, 0, -2, 2] as &[_], 0),
                &[[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]] as &[_],
            ),
            ((&[2, 2, 2, 2, 2], 8), &[[2, 2, 2, 2]]),
            ((&[], 0), &[]),
            ((&[0, 0, 0, 0], 0), &[[0, 0, 0, 0]]),
            ((&[0, 1, 5, 0, 1, 5, 5, -4], 11), &[[-4, 5, 5, 5], [0, 1, 5, 5]]),
            ((&[-3, -1, 0, 2, 4, 5], 0), &[[-3, -1, 0, 4]]),
            ((&[2, 1, 0, -1], 2), &[[-1, 0, 1, 2]]),
            (
                (&[-3, -2, -1, 0, 0, 1, 2, 3], 0),
                &[
                    [-3, -2, 2, 3],
                    [-3, -1, 1, 3],
                    [-3, 0, 0, 3],
                    [-3, 0, 1, 2],
                    [-2, -1, 0, 3],
                    [-2, -1, 1, 2],
                    [-2, 0, 0, 2],
                    [-1, 0, 0, 1],
                ],
            ),
            ((&[5, 5, 3, 5, 1, -5, 1, -2], 4), &[[-5, 1, 3, 5]]),
        ];

        for ((nums, target), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::four_sum(nums.to_vec(), target)),
                expected
            );
        }
    }
}

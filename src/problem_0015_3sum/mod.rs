pub mod sort_then_two_sum;
pub mod sort_then_two_sum_short;

pub trait Solution {
    fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[-1, 0, 1, 2, -1, -4] as &[_], &[&[-1, -1, 2], &[-1, 0, 1]] as &[_]),
            (&[], &[]),
            (
                &[-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6],
                &[
                    &[-4, -2, 6],
                    &[-4, 0, 4],
                    &[-4, 1, 3],
                    &[-4, 2, 2],
                    &[-2, -2, 4],
                    &[-2, 0, 2],
                ],
            ),
            (&[0; 3000], &[&[0, 0, 0]]),
            (&[-1, 0, 1], &[&[-1, 0, 1]]),
            (&[1, 1, 1], &[]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(test_utilities::unstable_sorted(S::three_sum(nums.to_vec())), expected);
        }
    }
}

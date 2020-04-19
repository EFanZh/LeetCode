pub mod sort_then_two_sum;
pub mod sort_then_two_sum_short;

pub trait Solution {
    fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn sorted<T: Ord>(v: Vec<T>) -> Vec<T> {
        let mut v = v;

        v.sort_unstable();

        v
    }

    pub fn run_tests<S: Solution>() {
        let test_cases = [
            (&[-1, 0, 1, 2, -1, -4] as &[_], &[&[-1, 0, 1], &[-1, -1, 2]] as &[_]),
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

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(sorted(S::three_sum(nums.to_owned())), sorted(expected.to_owned()));
        }
    }
}

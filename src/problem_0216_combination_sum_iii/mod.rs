pub mod recursive;
pub mod recursive_2;

pub trait Solution {
    fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((3, 7), &[&[1, 2, 4] as &[_]] as &[&[_]]),
            ((3, 9), &[&[1, 2, 6], &[1, 3, 5], &[2, 3, 4]]),
            ((4, 1), &[]),
            ((3, 2), &[]),
            ((9, 45), &[&[1, 2, 3, 4, 5, 6, 7, 8, 9]]),
        ];

        for ((k, n), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(
                    S::combination_sum3(k, n)
                        .into_iter()
                        .map(test_utilities::unstable_sorted)
                ),
                expected,
            );
        }
    }
}

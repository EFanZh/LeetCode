pub mod dfs;

pub trait Solution {
    fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((3, 7), &[181, 292, 707, 818, 929] as &[_]),
            (
                (2, 1),
                &[10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98],
            ),
            ((2, 0), &[11, 22, 33, 44, 55, 66, 77, 88, 99]),
        ];

        for ((n, k), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::nums_same_consec_diff(n, k)),
                expected
            );
        }
    }
}

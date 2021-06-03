pub mod iterative;

pub trait Solution {
    fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((3, 3, &[[2, 2], [3, 3]] as &[_]), 4),
            (
                (
                    3,
                    3,
                    &[
                        [2, 2],
                        [3, 3],
                        [3, 3],
                        [3, 3],
                        [2, 2],
                        [3, 3],
                        [3, 3],
                        [3, 3],
                        [2, 2],
                        [3, 3],
                        [3, 3],
                        [3, 3],
                    ],
                ),
                4,
            ),
            ((3, 3, &[]), 9),
        ];

        for ((m, n, ops), expected) in test_cases.iter().copied() {
            assert_eq!(
                S::max_count(m, n, ops.iter().copied().map(Vec::from).collect()),
                expected
            );
        }
    }
}

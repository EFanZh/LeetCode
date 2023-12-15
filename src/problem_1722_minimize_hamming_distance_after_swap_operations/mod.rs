pub mod union_find;

pub trait Solution {
    fn minimum_hamming_distance(source: Vec<i32>, target: Vec<i32>, allowed_swaps: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[1, 2, 3, 4] as &[_], &[2, 1, 4, 5] as &[_], &[[0, 1], [2, 3]] as &[_]),
                1,
            ),
            ((&[1, 2, 3, 4], &[1, 3, 2, 4], &[]), 2),
            (
                (&[5, 1, 2, 4, 3], &[1, 5, 4, 2, 3], &[[0, 4], [4, 2], [1, 3], [1, 4]]),
                0,
            ),
            (
                (&[71, 13, 6, 60, 22, 31], &[66, 57, 2, 60, 22, 73], &[[4, 5], [0, 4]]),
                4,
            ),
            (
                (
                    &[18, 67, 10, 36, 17, 62, 38, 78, 52],
                    &[3, 4, 99, 36, 26, 58, 29, 33, 74],
                    &[
                        [4, 7],
                        [3, 1],
                        [8, 4],
                        [5, 6],
                        [2, 8],
                        [0, 7],
                        [1, 6],
                        [3, 7],
                        [2, 5],
                        [3, 0],
                        [8, 5],
                        [2, 1],
                        [6, 7],
                        [5, 1],
                        [3, 6],
                        [4, 0],
                        [7, 2],
                        [2, 6],
                        [4, 1],
                        [3, 2],
                        [8, 6],
                        [8, 0],
                        [5, 3],
                        [1, 0],
                        [4, 6],
                        [8, 7],
                        [5, 7],
                        [3, 8],
                        [6, 0],
                        [8, 1],
                        [7, 1],
                        [5, 0],
                        [4, 3],
                        [0, 2],
                    ],
                ),
                8,
            ),
        ];

        for ((source, target, allowed_swaps), expected) in test_cases {
            assert_eq!(
                S::minimum_hamming_distance(
                    source.to_vec(),
                    target.to_vec(),
                    allowed_swaps.iter().map(Vec::from).collect()
                ),
                expected,
            );
        }
    }
}

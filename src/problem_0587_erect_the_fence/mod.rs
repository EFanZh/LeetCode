pub mod grahams_scan;

pub trait Solution {
    fn outer_trees(points: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[1, 1], [2, 2], [2, 0], [2, 4], [3, 3], [4, 2]] as &[_],
                &[[1, 1], [2, 0], [2, 4], [3, 3], [4, 2]] as &[_],
            ),
            (&[[1, 2], [2, 2], [4, 2]], &[[1, 2], [2, 2], [4, 2]]),
            (
                &[
                    [3, 0],
                    [4, 0],
                    [5, 0],
                    [6, 1],
                    [7, 2],
                    [7, 3],
                    [7, 4],
                    [6, 5],
                    [5, 5],
                    [4, 5],
                    [3, 5],
                    [2, 5],
                    [1, 4],
                    [1, 3],
                    [1, 2],
                    [2, 1],
                    [4, 2],
                    [0, 3],
                ],
                &[
                    [0, 3],
                    [1, 2],
                    [1, 4],
                    [2, 1],
                    [2, 5],
                    [3, 0],
                    [3, 5],
                    [4, 0],
                    [4, 5],
                    [5, 0],
                    [5, 5],
                    [6, 1],
                    [6, 5],
                    [7, 2],
                    [7, 3],
                    [7, 4],
                ],
            ),
            (
                &[[1, 2], [2, 2], [4, 2], [5, 2], [6, 2], [7, 2]],
                &[[1, 2], [2, 2], [4, 2], [5, 2], [6, 2], [7, 2]],
            ),
        ];

        for (points, expected) in test_cases.iter().copied() {
            assert_eq!(
                test_utilities::unstable_sorted(S::outer_trees(points.iter().copied().map(Vec::from).collect())),
                expected
            );
        }
    }
}

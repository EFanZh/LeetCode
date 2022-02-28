pub mod merge_by_columns;

pub trait Solution {
    fn min_area_rect(points: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 1], [1, 3], [3, 1], [3, 3], [2, 2]] as &[_], 4),
            (&[[1, 1], [1, 3], [3, 1], [3, 3], [4, 1], [4, 3]], 2),
            (&[[0, 1], [1, 3], [3, 3], [4, 4], [1, 4], [2, 3], [1, 0], [3, 4]], 2),
            (
                &[
                    [3, 2],
                    [1, 3],
                    [3, 3],
                    [3, 0],
                    [3, 1],
                    [2, 0],
                    [4, 2],
                    [1, 0],
                    [4, 1],
                    [1, 1],
                ],
                1,
            ),
            (&[[3, 2], [3, 1], [4, 4], [1, 1], [4, 3], [0, 3], [0, 2], [4, 0]], 0),
        ];

        for (points, expected) in test_cases {
            assert_eq!(
                S::min_area_rect(points.iter().copied().map(Vec::from).collect()),
                expected
            );
        }
    }
}

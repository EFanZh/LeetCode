pub mod count_corners;

pub trait Solution {
    fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[1, 1, 3, 3], [3, 1, 4, 2], [3, 2, 4, 4], [1, 3, 2, 4], [2, 3, 3, 4]] as &[_],
                true,
            ),
            (&[[1, 1, 2, 3], [1, 3, 2, 4], [3, 1, 4, 2], [3, 2, 4, 4]], false),
            (&[[1, 1, 3, 3], [3, 1, 4, 2], [1, 3, 2, 4], [3, 2, 4, 4]], false),
            (&[[1, 1, 3, 3], [3, 1, 4, 2], [1, 3, 2, 4], [2, 2, 4, 4]], false),
            (
                &[
                    [0, 0, 4, 1],
                    [7, 0, 8, 2],
                    [6, 2, 8, 3],
                    [5, 1, 6, 3],
                    [4, 0, 5, 1],
                    [6, 0, 7, 2],
                    [4, 2, 5, 3],
                    [2, 1, 4, 3],
                    [0, 1, 2, 2],
                    [0, 2, 2, 3],
                    [4, 1, 5, 2],
                    [5, 0, 6, 1],
                ],
                true,
            ),
        ];

        for (rectangles, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::is_rectangle_cover(rectangles.iter().map(|rectangle| rectangle.to_vec()).collect()),
                expected
            );
        }
    }
}

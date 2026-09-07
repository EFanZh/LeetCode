pub mod greedy;

pub trait Solution {
    fn check_valid_cuts(n: i32, rectangles: Vec<Vec<i32>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (5, &[[1, 0, 5, 2], [0, 2, 2, 4], [3, 2, 5, 3], [0, 4, 4, 5]] as &[_]),
                true,
            ),
            ((4, &[[0, 0, 1, 1], [2, 0, 3, 4], [0, 2, 2, 3], [3, 0, 4, 3]]), true),
            (
                (
                    4,
                    &[[0, 2, 2, 4], [1, 0, 3, 2], [2, 2, 3, 4], [3, 0, 4, 2], [3, 2, 4, 4]],
                ),
                false,
            ),
        ];

        for ((n, rectangles), expected) in test_cases {
            assert_eq!(
                S::check_valid_cuts(n, rectangles.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}

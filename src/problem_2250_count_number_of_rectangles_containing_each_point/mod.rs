pub mod dynamic_programming;

pub trait Solution {
    fn count_rectangles(rectangles: Vec<Vec<i32>>, points: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[[1, 2], [2, 3], [2, 5]] as &[_], &[[2, 1], [1, 4]] as &[_]),
                &[2, 1] as &[_],
            ),
            ((&[[1, 1], [2, 2], [3, 3]], &[[1, 3], [1, 1]]), &[1, 3]),
        ];

        for ((rectangles, points), expected) in test_cases {
            assert_eq!(
                S::count_rectangles(
                    rectangles.iter().map(Vec::from).collect(),
                    points.iter().map(Vec::from).collect(),
                ),
                expected,
            );
        }
    }
}

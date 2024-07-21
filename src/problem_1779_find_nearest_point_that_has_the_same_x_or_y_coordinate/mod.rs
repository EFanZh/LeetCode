pub mod iterative;

pub trait Solution {
    fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((3, 4, &[[1, 2], [3, 1], [2, 4], [2, 3], [4, 4]] as &[_]), 2),
            ((3, 4, &[[3, 4]]), 0),
            ((3, 4, &[[2, 3]]), -1),
        ];

        for ((x, y, points), expected) in test_cases {
            assert_eq!(
                S::nearest_valid_point(x, y, points.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}

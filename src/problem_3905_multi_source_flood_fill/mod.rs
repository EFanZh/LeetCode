pub mod bfs;

pub trait Solution {
    fn color_grid(n: i32, m: i32, sources: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (3, 3, &[[0, 0, 1], [2, 2, 2]] as &[_]),
                &[[1, 1, 2], [1, 2, 2], [2, 2, 2]] as &dyn Matrix<_>,
            ),
            ((3, 3, &[[0, 1, 3], [1, 1, 5]]), &[[3, 3, 3], [5, 5, 5], [5, 5, 5]]),
            ((2, 2, &[[1, 1, 5]]), &[[5, 5], [5, 5]]),
        ];

        for ((n, m, sources), expected) in test_cases {
            assert_eq!(S::color_grid(n, m, sources.iter().map(Vec::from).collect()), expected);
        }
    }
}

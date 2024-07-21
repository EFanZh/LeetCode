pub mod iterative;

pub trait Solution {
    fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (5, &[[0, 0], [4, 4]] as &[_], &[[1, 1], [1, 0]] as &[_]),
                &[1, 0] as &[_],
            ),
            ((5, &[[0, 0], [4, 4]], &[[1, 1], [1, 1]]), &[1, 1]),
            ((5, &[[0, 0], [0, 4]], &[[0, 4], [0, 1], [1, 4]]), &[1, 1, 0]),
        ];

        for ((n, lamps, queries), expected) in test_cases {
            assert_eq!(
                S::grid_illumination(
                    n,
                    lamps.iter().map(Vec::from).collect(),
                    queries.iter().map(Vec::from).collect()
                ),
                expected,
            );
        }
    }
}

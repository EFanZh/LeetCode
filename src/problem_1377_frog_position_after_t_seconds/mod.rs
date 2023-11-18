pub mod dfs;

pub trait Solution {
    fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (7, &[[1, 2], [1, 3], [1, 7], [2, 4], [2, 6], [3, 5]] as &[_], 2, 4),
                1.0 / 6.0,
            ),
            ((7, &[[1, 2], [1, 3], [1, 7], [2, 4], [2, 6], [3, 5]], 1, 7), 1.0 / 3.0),
            ((7, &[[1, 2], [1, 3], [1, 7], [2, 4], [2, 6], [3, 5]], 20, 6), 1.0 / 6.0),
            ((1, &[], 0, 1), 1.0),
            ((1, &[], 1, 1), 1.0),
            ((1, &[], 2, 1), 1.0),
            ((2, &[[1, 2]], 0, 1), 1.0),
            ((2, &[[1, 2]], 1, 1), 0.0),
            ((2, &[[1, 2]], 2, 1), 0.0),
            ((2, &[[1, 2]], 0, 2), 0.0),
            ((2, &[[1, 2]], 1, 2), 1.0),
            ((2, &[[1, 2]], 2, 2), 1.0),
        ];

        for ((n, edges, t, target), expected) in test_cases {
            approx::assert_ulps_eq!(
                S::frog_position(n, edges.iter().map(Vec::from).collect(), t, target),
                expected,
            );
        }
    }
}

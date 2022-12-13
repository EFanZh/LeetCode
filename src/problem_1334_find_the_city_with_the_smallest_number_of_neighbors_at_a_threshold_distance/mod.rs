pub mod floyd_warshall_algorithm;

pub trait Solution {
    fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((4, &[[0, 1, 3], [1, 2, 1], [1, 3, 4], [2, 3, 1]] as &[_], 4), 3),
            (
                (
                    5,
                    &[[0, 1, 2], [0, 4, 8], [1, 2, 3], [1, 4, 2], [2, 3, 1], [3, 4, 1]],
                    2,
                ),
                0,
            ),
        ];

        for ((n, edges, distance_threshold), expected) in test_cases {
            assert_eq!(
                S::find_the_city(n, edges.iter().copied().map(Vec::from).collect(), distance_threshold),
                expected
            );
        }
    }
}

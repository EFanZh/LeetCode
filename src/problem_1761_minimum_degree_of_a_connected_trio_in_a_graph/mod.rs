pub mod adjacency_matrix;
pub mod hash_set;

pub trait Solution {
    fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((6, &[[1, 2], [1, 3], [3, 2], [4, 1], [5, 2], [3, 6]] as &[_]), 3),
            (
                (7, &[[1, 3], [4, 1], [4, 3], [2, 5], [5, 6], [6, 7], [7, 5], [2, 6]]),
                0,
            ),
        ];

        for ((n, edges), expected) in test_cases {
            assert_eq!(S::min_trio_degree(n, edges.iter().map(Vec::from).collect()), expected,);
        }
    }
}

pub mod bfs;

pub trait Solution {
    fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[0, 1], [0, 2], [0, 3]] as &[_], &[[0, 1]] as &[_]), 3),
            (
                (
                    &[[0, 1], [0, 2], [0, 3], [2, 4], [2, 5], [3, 6], [2, 7]],
                    &[[0, 1], [0, 2], [0, 3], [2, 4], [2, 5], [3, 6], [2, 7]],
                ),
                5,
            ),
        ];

        for ((edges1, edges2), expected) in test_cases {
            assert_eq!(
                S::minimum_diameter_after_merge(
                    edges1.iter().map(Vec::from).collect(),
                    edges2.iter().map(Vec::from).collect()
                ),
                expected
            );
        }
    }
}

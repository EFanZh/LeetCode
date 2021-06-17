pub mod bfs;
pub mod bfs_2;

pub trait Solution {
    fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((4, &[[1, 0], [1, 2], [1, 3]] as &[_]), &[1] as &[_]),
            ((6, &[[3, 0], [3, 1], [3, 2], [3, 4], [5, 4]]), &[3, 4]),
            ((1, &[]), &[0]),
            ((2, &[[0, 1]]), &[0, 1]),
            ((7, &[[0, 1], [1, 2], [1, 3], [2, 4], [3, 5], [4, 6]]), &[1, 2]),
        ];

        for ((n, edges), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::find_min_height_trees(
                    n,
                    edges.iter().copied().map(Vec::from).collect()
                )),
                expected
            );
        }
    }
}

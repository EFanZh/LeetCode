pub mod dfs;
pub mod dfs_2;
pub mod dfs_3;

pub trait Solution {
    fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    8,
                    &[[0, 3], [0, 4], [1, 3], [2, 4], [2, 7], [3, 5], [3, 6], [3, 7], [4, 6]] as &[_],
                ),
                &[
                    &[] as &[_],
                    &[],
                    &[],
                    &[0, 1],
                    &[0, 2],
                    &[0, 1, 3],
                    &[0, 1, 2, 3, 4],
                    &[0, 1, 2, 3],
                ] as &[&[_]],
            ),
            (
                (
                    5,
                    &[
                        [0, 1],
                        [0, 2],
                        [0, 3],
                        [0, 4],
                        [1, 2],
                        [1, 3],
                        [1, 4],
                        [2, 3],
                        [2, 4],
                        [3, 4],
                    ],
                ),
                &[&[], &[0], &[0, 1], &[0, 1, 2], &[0, 1, 2, 3]],
            ),
        ];

        for ((n, edges), expected) in test_cases {
            assert_eq!(S::get_ancestors(n, edges.iter().map(Vec::from).collect()), expected);
        }
    }
}

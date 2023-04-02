pub mod floyd_warshall;
pub mod iterative_dfs;
pub mod recursive_dfs;

pub trait Solution {
    fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (2, &[[1, 0]] as &[_], &[[0, 1], [1, 0]] as &[_]),
                &[false, true] as &[_],
            ),
            ((2, &[], &[[1, 0], [0, 1]]), &[false, false]),
            ((3, &[[1, 2], [1, 0], [2, 0]], &[[1, 0], [1, 2]]), &[true, true]),
            (
                (
                    7,
                    &[
                        [2, 3],
                        [2, 1],
                        [2, 0],
                        [3, 4],
                        [3, 6],
                        [5, 1],
                        [5, 0],
                        [1, 4],
                        [1, 0],
                        [4, 0],
                        [0, 6],
                    ],
                    &[
                        [3, 0],
                        [6, 4],
                        [5, 6],
                        [2, 6],
                        [2, 3],
                        [5, 6],
                        [4, 0],
                        [2, 6],
                        [3, 5],
                        [5, 3],
                        [1, 6],
                        [1, 0],
                        [3, 5],
                        [6, 5],
                        [2, 3],
                        [3, 0],
                        [3, 4],
                        [3, 4],
                        [2, 5],
                        [0, 3],
                        [4, 0],
                        [6, 4],
                        [5, 0],
                        [6, 5],
                        [5, 6],
                        [6, 5],
                        [1, 0],
                        [3, 4],
                        [1, 5],
                        [1, 4],
                        [3, 6],
                        [0, 1],
                        [1, 2],
                        [5, 1],
                        [5, 3],
                        [5, 3],
                        [3, 4],
                        [5, 4],
                        [5, 4],
                        [5, 3],
                    ],
                ),
                &[
                    true, false, true, true, true, true, true, true, false, false, true, true, false, false, true,
                    true, true, true, false, false, true, false, true, false, true, false, true, true, false, true,
                    true, false, false, true, false, false, true, true, true, false,
                ],
            ),
        ];

        for ((num_courses, prerequisites, queries), expected) in test_cases {
            assert_eq!(
                S::check_if_prerequisite(
                    num_courses,
                    prerequisites.iter().copied().map(Vec::from).collect(),
                    queries.iter().copied().map(Vec::from).collect(),
                ),
                expected,
            );
        }
    }
}

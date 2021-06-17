pub mod bfs;
pub mod iterative_dfs;
pub mod iterative_dfs_2;
pub mod iterative_dfs_3;
pub mod recursive_dfs;
pub mod recursive_dfs_2;
pub mod recursive_dfs_3;

pub trait Solution {
    fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((2, &[[1, 0]] as &[_]), true),
            ((2, &[[1, 0], [0, 1]]), false),
            ((3, &[[1, 0], [2, 1]]), true),
            ((2, &[[0, 1]]), true),
            (
                (
                    20,
                    &[[0, 10], [3, 18], [5, 5], [6, 11], [11, 14], [13, 1], [15, 1], [17, 4]],
                ),
                false,
            ),
        ];

        for ((num_courses, prerequisites), expected) in test_cases {
            assert_eq!(
                S::can_finish(num_courses, prerequisites.iter().copied().map(Vec::from).collect()),
                expected
            );
        }
    }
}

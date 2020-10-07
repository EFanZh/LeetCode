pub mod bfs;
pub mod iterative_dfs;
pub mod iterative_dfs_2;
pub mod recursive_dfs;
pub mod recursive_dfs_2;

pub trait Solution {
    fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::collections::{HashMap, HashSet};

    pub fn run<S: Solution>() {
        let test_cases = [
            ((2, &[[1, 0]] as &[_]), true),
            ((4, &[[1, 0], [2, 0], [3, 1], [3, 2]]), true),
            ((1, &[]), true),
            ((2, &[[1, 0], [0, 1]]), false),
            ((3, &[[1, 0], [1, 2], [0, 1]]), false),
        ];

        for ((num_courses, prerequisites), can_finish) in test_cases.iter().copied() {
            let result = S::find_order(num_courses, prerequisites.iter().map(|edge| edge.to_vec()).collect());

            if can_finish {
                let deduped_result = result.iter().copied().collect::<HashSet<_>>();

                let course_indices = result
                    .into_iter()
                    .enumerate()
                    .map(|(i, x)| (x, i))
                    .collect::<HashMap<_, _>>();

                assert_eq!(deduped_result.len(), num_courses as _);

                for [course, prerequisite] in prerequisites {
                    assert!(course_indices[prerequisite] < course_indices[course])
                }
            } else {
                assert!(result.is_empty());
            }
        }
    }
}

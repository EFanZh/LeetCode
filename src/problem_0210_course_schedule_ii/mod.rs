pub mod in_degrees;
pub mod in_degrees_2;
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
    use std::collections::HashSet;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((2, &[[1, 0]] as &[_]), true),
            ((4, &[[1, 0], [2, 0], [3, 1], [3, 2]]), true),
            ((1, &[]), true),
            ((2, &[[1, 0], [0, 1]]), false),
            ((3, &[[1, 0], [1, 2], [0, 1]]), false),
            ((2, &[[0, 1]]), true),
        ];

        for ((num_courses, prerequisites), can_finish) in test_cases.iter().copied() {
            let result = S::find_order(num_courses, prerequisites.iter().copied().map(Vec::from).collect());

            if can_finish {
                assert_eq!(result.iter().copied().collect::<HashSet<_>>().len(), num_courses as _);

                let mut course_indices = vec![0; num_courses as _];

                for (i, node) in result.into_iter().enumerate() {
                    course_indices[node as usize] = i;
                }

                for &[course, prerequisite] in prerequisites {
                    assert!(course_indices[prerequisite as usize] < course_indices[course as usize]);
                }
            } else {
                assert!(result.is_empty());
            }
        }
    }
}

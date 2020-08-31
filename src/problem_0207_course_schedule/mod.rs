pub mod bfs;
pub mod recursive_dfs;

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
        ];

        for ((num_courses, prerequisites), expected) in test_cases.iter().copied() {
            assert_eq!(
                S::can_finish(num_courses, prerequisites.iter().map(|edge| edge.to_vec()).collect()),
                expected
            );
        }
    }
}

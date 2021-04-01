pub mod dynamic_programming;

pub trait Solution {
    fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[&[0, 0, 0] as &[_], &[0, 1, 0], &[0, 0, 0]] as &[&[_]], 2)];

        for (obstacle_grid, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::unique_paths_with_obstacles(obstacle_grid.iter().copied().map(<[_]>::to_vec).collect()),
                expected
            );
        }
    }
}

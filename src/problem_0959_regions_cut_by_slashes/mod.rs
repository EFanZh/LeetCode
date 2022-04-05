pub mod bfs;

pub trait Solution {
    fn regions_by_slashes(grid: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[" /", "/ "] as &[_], 2), (&[" /", "  "], 1), (&["/\\", "\\/"], 5)];

        for (grid, expected) in test_cases {
            assert_eq!(
                S::regions_by_slashes(grid.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}

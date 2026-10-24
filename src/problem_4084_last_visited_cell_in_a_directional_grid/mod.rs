pub mod iterative;

pub trait Solution {
    fn last_visited_cell(grid: Vec<Vec<char>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&["RD", "UL"] as &[_], [1, 0]), (&["RRR"], [0, 2])];

        for (grid, expected) in test_cases {
            assert_eq!(
                S::last_visited_cell(grid.iter().map(|row| row.chars().collect()).collect()),
                expected,
            );
        }
    }
}

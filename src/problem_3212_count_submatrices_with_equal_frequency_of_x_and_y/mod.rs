pub mod dynamic_programming;

pub trait Solution {
    fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&["XY.", "Y.."] as &[_], 3), (&["XX", "XY"], 0), (&["..", ".."], 0)];

        for (grid, expected) in test_cases {
            assert_eq!(
                S::number_of_submatrices(grid.iter().map(|row| row.chars().collect()).collect()),
                expected,
            );
        }
    }
}

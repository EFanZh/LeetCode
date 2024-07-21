pub mod dynamic_programming;

pub trait Solution {
    fn paths_with_max_score(board: Vec<String>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["E23", "2X2", "12S"] as &[_], [7, 1]),
            (&["E12", "1X1", "21S"], [4, 2]),
            (&["E11", "XXX", "11S"], [0, 0]),
        ];

        for (board, expected) in test_cases {
            assert_eq!(
                S::paths_with_max_score(board.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}

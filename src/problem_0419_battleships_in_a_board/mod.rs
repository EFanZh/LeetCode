pub mod count_heads;

pub trait Solution {
    fn count_battleships(board: Vec<Vec<char>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["X..X", "...X", "...X"] as &[_], 2),
            (&["."], 0),
            (&["XXX"], 1),
            (&["XX", ".."], 1),
        ];

        for (board, expected) in test_cases {
            assert_eq!(
                S::count_battleships(board.iter().map(|row| row.chars().collect()).collect()),
                expected
            );
        }
    }
}

pub mod brute_force;

pub trait Solution {
    fn can_make_square(grid: Vec<Vec<char>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (["BWB", "BWW", "BWB"], true),
            (["BWB", "WBW", "BWB"], false),
            (["BWB", "BWW", "BWW"], true),
            (["BBB", "BBB", "BBB"], true),
        ];

        for (board, expected) in test_cases {
            assert_eq!(
                S::can_make_square(board.iter().map(|row| row.chars().collect()).collect()),
                expected,
            );
        }
    }
}

pub mod dfs;

pub trait Solution {
    fn contains_cycle(grid: Vec<Vec<char>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["aaaa", "abba", "abba", "aaaa"] as &[_], true),
            (&["ccca", "cdcc", "ccec", "fccc"], true),
            (&["abb", "bzb", "bba"], false),
        ];

        for (board, expected) in test_cases {
            assert_eq!(
                S::contains_cycle(board.iter().map(|row| row.chars().collect()).collect()),
                expected,
            );
        }
    }
}

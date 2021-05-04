pub mod bfs;

pub trait Solution {
    fn find_min_step(board: String, hand: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("WRRBBW", "RB"), -1),
            (("WWRRBBWW", "WRBRW"), 2),
            (("G", "GGGGG"), 2),
            (("RBYYBBRRB", "YRBGB"), 3),
            (("RRWWRRBBRR", "WB"), 2),
            (("BB", "B"), 1),
        ];

        for ((board, hand), expected) in test_cases.iter().copied() {
            assert_eq!(S::find_min_step(board.to_string(), hand.to_string()), expected);
        }
    }
}

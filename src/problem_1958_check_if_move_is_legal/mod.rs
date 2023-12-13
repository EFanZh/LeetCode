pub mod iterative;

pub trait Solution {
    fn check_move(board: Vec<Vec<char>>, r_move: i32, c_move: i32, color: char) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    [
                        "...B....", "...W....", "...W....", "...W....", "WBB.WWWB", "...B....", "...B....", "...W....",
                    ],
                    4,
                    3,
                    'B',
                ),
                true,
            ),
            (
                (
                    [
                        "........", ".B..W...", "..W.....", "...WB...", "........", "....BW..", "......W.", ".......B",
                    ],
                    4,
                    4,
                    'W',
                ),
                false,
            ),
        ];

        for ((board, r_move, c_move, color), expected) in test_cases {
            assert_eq!(
                S::check_move(
                    board.iter().map(|row| row.chars().collect()).collect(),
                    r_move,
                    c_move,
                    color,
                ),
                expected,
            );
        }
    }
}

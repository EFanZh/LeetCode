pub mod brute_force;

pub trait Solution {
    fn num_rook_captures(board: Vec<Vec<char>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                [
                    "........", "...p....", "...R...p", "........", "........", "...p....", "........", "........",
                ],
                3,
            ),
            (
                [
                    "........", ".ppppp..", ".ppBpp..", ".pBRBp..", ".ppBpp..", ".ppppp..", "........", "........",
                ],
                0,
            ),
            (
                [
                    "........", "...p....", "...p....", "pp.R.pB.", "........", "...B....", "...p....", "........",
                ],
                3,
            ),
        ];

        for (board, expected) in test_cases {
            assert_eq!(
                S::num_rook_captures(board.iter().map(|row| row.chars().collect()).collect()),
                expected
            );
        }
    }
}

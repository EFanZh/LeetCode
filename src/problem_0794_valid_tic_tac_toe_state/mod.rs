pub mod brute_force;

pub trait Solution {
    fn valid_tic_tac_toe(board: Vec<String>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (["O  ", "   ", "   "], false),
            (["OOO", "XXO", "XXX"], false),
            (["OXX", "XOX", "OXO"], false),
            (["XOX", " X ", "   "], false),
            (["XOX", "O O", "XOX"], true),
            (["XOX", "O X", "X O"], true),
            (["XOX", "OOX", "XO "], true),
            (["XXX", "   ", "OOO"], false),
            (["XXX", "OOX", "OOX"], true),
            (["XXX", "XOO", "OO "], false),
        ];

        for (board, expected) in test_cases {
            assert_eq!(S::valid_tic_tac_toe(board.map(str::to_string).into()), expected);
        }
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper(moves: Vec<Vec<i32>>) -> &'static str {
        let n = moves.len();
        let mut board = (0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8);
        let mut player = 1;

        for m in moves {
            let [row, column]: [_; 2] = m.try_into().unwrap();

            match row * 3 + column {
                0 => board.0 = player,
                1 => board.1 = player,
                2 => board.2 = player,
                3 => board.3 = player,
                4 => board.4 = player,
                5 => board.5 = player,
                6 => board.6 = player,
                7 => board.7 = player,
                _ => board.8 = player,
            }

            player = 3 - player;
        }

        for (player, player_name) in [(1, "A"), (2, "B")] {
            for (a, b, c) in [
                (board.0, board.1, board.2),
                (board.0, board.3, board.6),
                (board.0, board.4, board.8),
                (board.1, board.4, board.7),
                (board.2, board.4, board.6),
                (board.2, board.5, board.8),
                (board.3, board.4, board.5),
                (board.6, board.7, board.8),
            ] {
                if a == player && b == player && c == player {
                    return player_name;
                }
            }
        }

        if n < 9 {
            "Pending"
        } else {
            "Draw"
        }
    }

    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        Self::helper(moves).to_string()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        Self::tictactoe(moves)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

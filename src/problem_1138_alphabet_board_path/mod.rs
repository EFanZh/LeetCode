pub mod iterative;

pub trait Solution {
    fn alphabet_board_path(target: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("leet", 15), ("code", 14)];

        let board = ["abcde", "fghij", "klmno", "pqrst", "uvwxy", "z"];
        let mut buffer = String::new();

        for (target, expected) in test_cases {
            let result = S::alphabet_board_path(target.to_string());

            assert_eq!(result.len(), expected);

            let mut row = 0;
            let mut column = 0;

            for c in result.bytes() {
                match c {
                    b'U' => row -= 1,
                    b'D' => row += 1,
                    b'L' => column -= 1,
                    b'R' => column += 1,
                    _ => buffer.push(char::from(board[row].as_bytes()[column])),
                }
            }

            assert_eq!(buffer, target);

            buffer.clear();
        }
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn check(mut line_iter: impl Iterator<Item = u8>, word: &[u8], tag: u8) -> bool {
        'outer: loop {
            let mut word_iter = word.iter().copied();

            loop {
                let c = word_iter.next();

                if let Some(pattern) = line_iter.next() {
                    match pattern {
                        b' ' => {
                            if c.is_some() {
                                continue;
                            }
                        }
                        b'#' => {
                            if c.is_none() {
                                return true;
                            }

                            break;
                        }
                        _ => {
                            if c == Some(pattern) {
                                continue;
                            }
                        }
                    }
                } else {
                    return c.is_none();
                }

                loop {
                    match line_iter.next() {
                        None => return false,
                        Some(b'#') => continue 'outer,
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn place_word_in_crossword(board: Vec<Vec<char>>, word: String) -> bool {
        let rows = board.len();
        let columns = board.first().map_or(0, Vec::len);
        let mut flat_board = Vec::with_capacity(columns * rows);

        for row in board {
            flat_board.extend(row.into_iter().map(|c| c as u8));
        }

        let word = word.as_bytes();

        for row in flat_board.chunks_exact(columns) {
            if Self::check(row.iter().copied(), word, 0) || Self::check(row.iter().copied().rev(), word, 1) {
                return true;
            }
        }

        let mut iter = flat_board.iter().copied();

        for _ in 0..columns {
            let column_iter = iter.clone().step_by(columns);

            if Self::check(column_iter.clone(), word, 2) || Self::check(column_iter.rev(), word, 3) {
                return true;
            }

            iter.next();
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn place_word_in_crossword(board: Vec<Vec<char>>, word: String) -> bool {
        Self::place_word_in_crossword(board, word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

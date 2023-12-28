pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        let mut length = 0_u32;
        let mut iter = colors.bytes();
        let mut alice_moves = 0;
        let mut bob_moves = 0;

        'outer: loop {
            if let Some(c) = iter.next() {
                if c == b'A' {
                    length += 1;
                } else {
                    alice_moves += length.saturating_sub(2);
                    length = 1;

                    loop {
                        if let Some(c) = iter.next() {
                            if c == b'A' {
                                bob_moves += length.saturating_sub(2);
                                length = 1;

                                break;
                            }

                            length += 1;
                        } else {
                            bob_moves += length.saturating_sub(2);

                            break 'outer;
                        }
                    }
                }
            } else {
                alice_moves += length.saturating_sub(2);

                break;
            }
        }

        alice_moves > bob_moves
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn winner_of_game(colors: String) -> bool {
        Self::winner_of_game(colors)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

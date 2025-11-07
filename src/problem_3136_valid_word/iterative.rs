pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_valid(word: String) -> bool {
        word.len() > 2 && {
            word.bytes().try_fold(0_u8, |state, c| {
                let bit = match c {
                    b'0'..=b'9' => 0,
                    b'A' | b'E' | b'I' | b'O' | b'U' | b'a' | b'e' | b'i' | b'o' | b'u' => 1,
                    b'B' | b'C' | b'D' | b'F' | b'G' | b'H' | b'J' | b'K' | b'L' | b'M' | b'N' | b'P' | b'Q' | b'R'
                    | b'S' | b'T' | b'V' | b'W' | b'X' | b'Y' | b'Z' | b'b' | b'c' | b'd' | b'f' | b'g' | b'h'
                    | b'j' | b'k' | b'l' | b'm' | b'n' | b'p' | b'q' | b'r' | b's' | b't' | b'v' | b'w' | b'x'
                    | b'y' | b'z' => 2,
                    _ => return None,
                };

                Some(state | bit)
            }) == Some(3)
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_valid(word: String) -> bool {
        Self::is_valid(word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

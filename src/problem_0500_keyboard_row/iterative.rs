pub struct Solution;

impl Solution {
    fn get_row(c: u8) -> u8 {
        match c {
            b'Z' | b'X' | b'C' | b'V' | b'B' | b'N' | b'M' | b'z' | b'x' | b'c' | b'v' | b'b' | b'n' | b'm' => 3,
            b'A' | b'S' | b'D' | b'F' | b'G' | b'H' | b'J' | b'K' | b'L' | b'a' | b's' | b'd' | b'f' | b'g' | b'h'
            | b'j' | b'k' | b'l' => 2,
            _ => 0,
        }
    }

    pub fn find_words(mut words: Vec<String>) -> Vec<String> {
        words.retain(|word| {
            let (&first, rest) = word.as_bytes().split_first().unwrap();
            let row = Self::get_row(first);

            rest.iter().all(|&c| Self::get_row(c) == row)
        });

        words
    }
}

impl super::Solution for Solution {
    fn find_words(words: Vec<String>) -> Vec<String> {
        Self::find_words(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

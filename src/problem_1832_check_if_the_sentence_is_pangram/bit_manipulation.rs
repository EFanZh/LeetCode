pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut state = 0;

        for c in sentence.bytes() {
            state |= 1 << (c - b'a');

            if state == (1 << 26) - 1 {
                return true;
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_if_pangram(sentence: String) -> bool {
        Self::check_if_pangram(sentence)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

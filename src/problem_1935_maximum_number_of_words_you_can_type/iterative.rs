pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let broken_letters = broken_letters
            .into_bytes()
            .into_iter()
            .fold(0_u32, |x, c| x | (1 << (c - b'a')));

        let mut ok = true;
        let mut result = 0;

        for c in text.into_bytes() {
            if c == b' ' {
                result += i32::from(ok);
                ok = true;
            } else {
                ok &= (1 << (c - b'a')) & broken_letters == 0;
            }
        }

        result += i32::from(ok);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        Self::can_be_typed_words(text, broken_letters)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

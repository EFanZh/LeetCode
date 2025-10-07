pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn add_minimum(word: String) -> i32 {
        let mut prev = b'c';
        let mut result = 0;

        for c in word.bytes() {
            let mut expected = prev + 1;

            if expected == b'd' {
                expected = b'a';
            }

            result += i32::from(c.checked_sub(expected).unwrap_or(c + 3 - expected));
            prev = c;
        }

        result += i32::from(b'c' - prev);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn add_minimum(word: String) -> i32 {
        Self::add_minimum(word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
